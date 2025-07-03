use std::path::PathBuf;

use serde::Deserialize;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The provided config file path does not exist
    #[error("Config file not found at: {0}")]
    NotFound(PathBuf),
    /// The config file is not valid YAML
    #[error("Could not parse config file: {0}")]
    Serde(#[from] serde_yaml::Error),
    /// An IO error occurred
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct VersionCleaning {
    pub trim_prefix: Option<String>,
    pub trim_suffix: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Source {
    Github {
        repository: String,
        #[serde(default = "default_github_source")]
        github_source: String,
        #[serde(default)]
        cleaning: VersionCleaning,
    },
}

fn default_github_source() -> String {
    "releases".to_string()
}

#[derive(Debug, Deserialize, Clone)]
pub struct Target {
    pub name: String,
    pub enabled: bool,
    #[serde(flatten)]
    pub source: Option<Source>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub targets: Vec<Target>,
    #[serde(default)]
    pub github_token: Option<String>,
}

/// Loads the configuration from the given path.
pub fn load(config_path: &Path) -> Result<Settings, Error> {
    if !config_path.exists() {
        return Err(Error::NotFound(config_path.to_path_buf()));
    }
    let contents = std::fs::read_to_string(config_path)?;
    let settings: Settings = serde_yaml::from_str(&contents)?;
    Ok(settings)
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
