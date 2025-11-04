//! 配置模块

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub base_branch: String,
    pub api_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_branch: "main".to_string(),
            api_key: None,
        }
    }
}
