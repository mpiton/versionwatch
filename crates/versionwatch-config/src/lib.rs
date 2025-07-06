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

#[derive(Debug, Deserialize, Clone)]
pub struct Target {
    pub name: String,
    pub enabled: bool,
    pub repository: Option<String>,
    #[serde(default = "default_github_source")]
    pub github_source: String,
    #[serde(default)]
    pub cleaning: VersionCleaning,
}

fn default_github_source() -> String {
    "releases".to_string()
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct VersionCleaning {
    #[serde(default)]
    pub enabled: bool,
    pub regex: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub targets: Vec<Target>,
    #[serde(default)]
    pub github_token: Option<String>,
}

/// Loads the configuration from the given path.
pub fn load(config_path: &Path) -> Result<Settings, Error> {
    dotenvy::dotenv().ok();
    if !config_path.exists() {
        return Err(Error::NotFound(config_path.to_path_buf()));
    }
    let contents = std::fs::read_to_string(config_path)?;
    let mut settings: Settings = serde_yaml::from_str(&contents)?;

    if settings.github_token.is_none() {
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            settings.github_token = Some(token);
        }
    }

    Ok(settings)
}
