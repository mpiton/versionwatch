use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to load configuration")]
    Load(#[from] config::ConfigError),
}

#[derive(Debug, Deserialize)]
pub struct Target {
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub targets: Vec<Target>,
    #[serde(default)]
    pub github_token: Option<String>,
}

pub fn load() -> Result<Settings, Error> {
    let settings = config::Config::builder()
        // Start off by merging in the "default" configuration file
        .add_source(config::File::with_name("config/base"))
        .add_source(config::Environment::with_prefix("VERSIONWATCH"))
        .build()?;

    Ok(settings.try_deserialize()?)
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
