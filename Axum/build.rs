const PAGES_DIR: &'static str = "src/pages";
const ROUTER_PATH: &'static str = "src/router.rs";

use anyhow::{anyhow, Result};
use rust_format::{Formatter, RustFmt};
use std::path::PathBuf;
use syn::{ItemFn, LitStr};

#[derive(Debug, Clone)]
struct PageEntry {
    name: String,
    is_dir: bool,
    children: Vec<PageEntry>,
}

#[derive(Debug, Clone)]
struct FunctionRoute {
    function: String,
    route_type: RouteType,
    route: Option<String>,
}

#[derive(Debug, Clone)]
enum RouteType {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Any,
}

impl RouteType {
    pub fn from_str(s: &str) -> Result<RouteType> {
        match s {
            "get" => Ok(RouteType::Get),
            "post" => Ok(RouteType::Post),
            "put" => Ok(RouteType::Put),
            "delete" => Ok(RouteType::Delete),
            "patch" => Ok(RouteType::Patch),
            "head" => Ok(RouteType::Head),
            "options" => Ok(RouteType::Options),
            "any" => Ok(RouteType::Any),
            _ => Err(anyhow!("Invalid route type")),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            RouteType::Get => "get",
            RouteType::Post => "post",
            RouteType::Put => "put",
            RouteType::Delete => "delete",
            RouteType::Patch => "patch",
            RouteType::Head => "head",
            RouteType::Options => "options",
            RouteType::Any => "any",
        }
    }
}

fn main() -> Result<()> {
    let pages = PathBuf::from(PAGES_DIR);

    let entries: PageEntry = PageEntry::generate(&pages)?;
    let lib_str = entries.generate_mods()?;
    let routes_str = entries.generate_routes(&pages);

    let routes = format!(
        r#"
        pub async fn router() -> Router {{
            let router = Router::new()
            {};

            return router
        }}
        "#,
        routes_str
    );

    let mut main_template_content = format!(
        r#"
            //THIS FILE IS AUTOGENERATED, DO NOT EDIT
            use axum::{{routing::{{any, delete, get, head, options, patch, post, put, trace}}, Router}};

            #[path = "pages"]
            {}

            {}
        "#,
        lib_str, routes
    );

    main_template_content = RustFmt::new().format_str(main_template_content)?;
    std::fs::write(ROUTER_PATH, main_template_content)?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=fnstack.json");
    println!("cargo:rerun-if-changed=src/pages");

    Ok(())
}

impl PageEntry {
    pub fn generate(dir: &PathBuf) -> Result<PageEntry> {
        let mut children: Vec<PageEntry> = Vec::new();

        for entry in dir.read_dir()? {
            if let Ok(entry) = entry {
                let file_type = entry.file_type()?;

                if file_type.is_dir() {
                    children.push(PageEntry::generate(&entry.path())?);
                } else if file_type.is_file() {
                    let file_name = entry
                        .file_name()
                        .to_str()
                        .unwrap()
                        .split('.')
                        .collect::<Vec<&str>>()[0]
                        .to_owned();

                    children.push(PageEntry {
                        name: file_name,
                        is_dir: entry.file_type()?.is_dir(),
                        children: vec![],
                    })
                }
            }
        }

        // WTF???
        let dir_name = dir.file_name().unwrap().to_str().unwrap().to_owned();
        children.sort_by_key(|k| k.is_dir);

        return Ok(PageEntry {
            name: dir_name,
            is_dir: true,
            children,
        });
    }

    pub fn generate_mods(&self) -> Result<String> {
        let mut out = String::new();
        if self.is_dir && self.children.len() == 0 {
            return Ok(out);
        }

        if self.name.contains(":") {
            out += &format!("#[path = \"{}\"]\n", self.name);
        }
        out += &format!("pub mod {}", self.name.replace(":", "_"));
        if self.children.len() > 0 {
            out += "{ \n";

            for child in self.children.clone() {
                out += &child.generate_mods()?;
            }

            out += "} \n";
        } else {
            out += "; \n";
        }

        Ok(out)
    }

    pub fn generate_routes(&self, path: &PathBuf) -> String {
        let mut tmp = String::new();

        for child in &self.children {
            let mut child_path = path.join(&child.name);
            if child.is_dir {
                tmp += &child.generate_routes(&child_path);
                continue;
            }

            child_path.set_extension("rs");
            let use_path = path
                .to_str()
                .unwrap()
                .replacen("src/", "", 1)
                .replace(":", "_")
                .replace("/", "::");
            let use_path = format!("{}::{}", use_path, child.name);

            let route_path = path
                .to_str()
                .unwrap()
                .replacen("src/pages", "", 1)
                .to_owned();

            for route in PageEntry::get_axum_routes(child_path).unwrap_or(vec![]) {
                let route_path =
                    route_path.clone() + "/" + &route.route.unwrap_or(String::from(""));
                let mut route_path = route_path.trim_end_matches("/");
                if route_path == "" {
                    route_path = "/";
                }

                tmp += &format!(
                    ".route(\"{}\", {}({}::{}))",
                    route_path,
                    route.route_type.to_str(),
                    use_path,
                    route.function
                );
            }
        }

        return tmp;
    }

    pub fn get_axum_routes(path: PathBuf) -> Result<Vec<FunctionRoute>> {
        let file_content = std::fs::read_to_string(path)?;

        let syntax = syn::parse_file(&file_content).unwrap();
        let functions: Vec<FunctionRoute> = syntax
            .items
            .iter()
            .filter_map(|item| {
                if let syn::Item::Fn(item_fn) = item {
                    if let Ok((route, route_type)) = PageEntry::is_axum_attr(item_fn) {
                        let fn_route = FunctionRoute {
                            function: item_fn.sig.ident.to_string(),
                            route_type,
                            route,
                        };

                        return Some(fn_route);
                    }
                }

                None
            })
            .collect();

        return Ok(functions);
    }

    const AXUM_MACROS: [&'static str; 9] = [
        "any", "get", "post", "put", "delete", "patch", "head", "options", "trace",
    ];
    fn is_axum_attr(item: &ItemFn) -> Result<(Option<String>, RouteType)> {
        for attr in item.attrs.clone() {
            for segment in attr.path().segments.clone() {
                let ident = segment.ident.to_string();
                if PageEntry::AXUM_MACROS.contains(&ident.as_str()) {
                    if let Ok(arg) = attr.parse_args::<LitStr>() {
                        return Ok((Some(arg.value()), RouteType::from_str(&ident)?));
                    }

                    return Ok((None, RouteType::from_str(&ident)?));
                }
            }
        }

        return Err(anyhow!(""));
    }
}
