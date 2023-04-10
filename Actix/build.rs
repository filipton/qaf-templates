const PAGES_DIR: &'static str = "src/pages";
const SCOPE_PATH: &'static str = "src/actix_scope.rs";

use anyhow::Result;
use fnstack_build_utils::PageEntry;
use rust_format::{Formatter, RustFmt};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn main() -> Result<()> {
    let config: BuildrsConfig = BuildrsConfig::from_file(PathBuf::from("fnstack.json"))?;
    let pages = PathBuf::from(PAGES_DIR);

    let entries: PageEntry = PageEntry::generate(&pages)?;
    let mods_str = entries.get_mods_string()?;
    let services_str = generate_services(&entries, &config);

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
        mods_str, services
    );

    main_template_content = RustFmt::new().format_str(main_template_content)?;
    std::fs::write(SCOPE_PATH, main_template_content)?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=fnstack.json");
    println!("cargo:rerun-if-changed=src/pages");

    Ok(())
}

// TODO: Refactor this
pub fn generate_services(entry: &PageEntry, config: &BuildrsConfig) -> String {
    let mut tmp = String::from("web::scope(\"\")");

    for child in &entry.children {
        if child.children.len() > 0 {
            tmp += &_generate_services(&child, PathBuf::from("src/pages"), config);
            continue;
        }

        let child_path = PathBuf::from(PAGES_DIR).join(format!("{}.rs", child.name));

        for route in fnstack_build_utils::get_file_routes(child_path).unwrap_or(vec![]) {
            tmp += &format!(
                ".service(pages::{}::{})\n",
                child.name.replace("{", "_").replace("}", "_"),
                route.function
            );
        }
    }

    return tmp;
}

fn _generate_services(entry: &PageEntry, path: PathBuf, config: &BuildrsConfig) -> String {
    let mut tmp = String::new();
    let path = path.join(&entry.name);

    if !config.disable_scopes {
        tmp += &format!(".service(web::scope(\"{}\")\n", entry.name);
    }

    for child in &entry.children {
        if child.children.len() > 0 {
            tmp += &_generate_services(&child, path.clone(), config);
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

        for route in fnstack_build_utils::get_file_routes(tmp_path).unwrap_or(vec![]) {
            tmp += &format!(
                ".service({}::{}::{})\n",
                use_path,
                child.name.replace("{", "_").replace("}", "_"),
                route.function
            );
        }
    }

    if !config.disable_scopes {
        tmp += ")";
    }
    return tmp;
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
