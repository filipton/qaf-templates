const PAGES_DIR: &'static str = "src/pages";
const SCOPE_PATH: &'static str = "src/actix_scope.rs";

use anyhow::Result;
use rust_format::{Formatter, RustFmt};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use syn::ItemFn;

#[derive(Debug, Clone)]
struct PageEntry {
    name: String,
    is_dir: bool,
    children: Vec<PageEntry>,
}

fn main() -> Result<()> {
    let config: BuildrsConfig = BuildrsConfig::from_file(PathBuf::from("fnstack.json"))?;

    let pages = PathBuf::from(PAGES_DIR);

    let entries: PageEntry = PageEntry::generate(pages, &config)?;
    let lib_str = entries.generate_mods()?;
    let services_str = entries.generate_services(&config);

    let services = format!(
        r#"
        pub fn generated_scope() -> actix_web::Scope {{
            {}
        }}
        "#,
        services_str
    );

    let mut main_template_content = format!(
        r#"
            //THIS FILE IS AUTOGENERATED, DO NOT EDIT
            use actix_web::web;
            
            #[path = "pages"]
            {}

            {}
        "#,
        lib_str, services
    );

    main_template_content = RustFmt::new().format_str(main_template_content)?;
    std::fs::write(SCOPE_PATH, main_template_content)?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=fnstack.json");
    println!("cargo:rerun-if-changed=src/pages");

    Ok(())
}

impl PageEntry {
    pub fn generate(dir: PathBuf, config: &BuildrsConfig) -> Result<PageEntry> {
        let mut children: Vec<PageEntry> = Vec::new();

        for entry in dir.read_dir()? {
            if let Ok(entry) = entry {
                let file_type = entry.file_type()?;

                if file_type.is_dir() {
                    children.push(PageEntry::generate(entry.path(), config)?);
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

        if self.name.contains("{") || self.name.contains("}") {
            out += &format!("#[path = \"{}\"]\n", self.name);
        }
        out += &format!("pub mod {}", self.name.replace("{", "_").replace("}", "_"));
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

    pub fn generate_services(&self, config: &BuildrsConfig) -> String {
        let mut tmp = String::from("web::scope(\"\")");

        for child in &self.children {
            if child.children.len() > 0 {
                tmp += &*child._generate_services(PathBuf::from("src/pages"), config);
                continue;
            }

            let child_path = PathBuf::from(PAGES_DIR).join(format!("{}.rs", child.name));
            for endpoint in PageEntry::get_actix_endpoints(child_path).unwrap_or(vec![]) {
                tmp += &format!(
                    ".service(pages::{}::{})",
                    child.name.replace("{", "_").replace("}", "_"),
                    endpoint
                );
            }
        }

        return tmp;
    }

    fn _generate_services(&self, path: PathBuf, config: &BuildrsConfig) -> String {
        let mut tmp = String::new();
        let path = path.join(&self.name);

        if !config.disable_scopes {
            tmp += &format!(".service(web::scope(\"{}\")\n", self.name);
        }

        for child in self.children.clone() {
            if child.children.len() > 0 {
                tmp += &child._generate_services(path.clone(), config);
                continue;
            }

            let tmp_path = path.clone().join(format!("{}.rs", child.name));
            let use_path = path
                .to_str()
                .unwrap()
                .replacen("src/", "", 1)
                .replace("/", "::")
                .replace("{", "_")
                .replace("}", "_");

            for endpoint in PageEntry::get_actix_endpoints(tmp_path).unwrap_or(vec![]) {
                tmp += &format!(".service({}::{}::{})\n", use_path, child.name, endpoint);
            }
        }

        if !config.disable_scopes {
            tmp += ")";
        }
        return tmp;
    }

    pub fn get_actix_endpoints(path: PathBuf) -> Result<Vec<String>> {
        let file_content = std::fs::read_to_string(path)?;

        let syntax = syn::parse_file(&file_content).unwrap();
        let functions: Vec<String> = syntax
            .items
            .iter()
            .filter_map(|item| {
                if let syn::Item::Fn(item_fn) = item {
                    if PageEntry::is_actix_attr(item_fn) {
                        return Some(item_fn.sig.ident.to_string());
                    }
                }

                None
            })
            .collect();

        return Ok(functions);
    }

    const ACTIX_MACROS: [&'static str; 7] =
        ["get", "post", "put", "delete", "head", "options", "patch"];

    fn is_actix_attr(item: &ItemFn) -> bool {
        for attr in item.attrs.clone() {
            for segment in attr.path.segments {
                let ident = segment.ident.to_string();
                if PageEntry::ACTIX_MACROS.contains(&ident.as_str()) {
                    return true;
                }
            }
        }

        return false;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildrsConfig {
    pub disable_scopes: bool,
}

impl BuildrsConfig {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&file)?;

        Ok(config)
    }
}
