use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildrsConfig {
    pub filenames_as_scopes: bool,
    pub disable_scopes: bool,
}

impl Default for BuildrsConfig {
    fn default() -> Self {
        Self {
            filenames_as_scopes: false,
            disable_scopes: false,
        }
    }
}

impl BuildrsConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
        let file = std::fs::read_to_string(path);

        if file.is_err() {
            return Ok(Self::new());
        }
        let config = serde_json::from_str(&file?)?;

        Ok(config)
    }

    pub fn to_file(data: Self, path: PathBuf) -> Result<()> {
        let config = serde_json::to_string_pretty(&data)?;
        std::fs::write(path, config)?;

        Ok(())
    }
}
