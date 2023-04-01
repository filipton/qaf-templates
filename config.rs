use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildrsConfig {
    pub filenames_as_scopes: bool,
    pub disable_scopes: bool,
    pub tmux_single_window: bool,
}

impl BuildrsConfig {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&file)?;

        Ok(config)
    }
}
