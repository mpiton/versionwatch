use async_trait::async_trait;
use polars::prelude::*;

pub mod apache;
pub mod docker;
pub mod eclipse_temurin;
pub mod elixir;
pub mod go;
pub mod kong;
pub mod kotlin;
pub mod nginx;
pub mod node;
pub mod perl;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod scala;
pub mod swift;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to fetch data from remote: {0}")]
    Fetch(#[from] reqwest::Error),
    #[error("Failed to parse JSON data: {0}")]
    ParseJson(#[from] serde_json::Error),
    #[error("Failed to parse YAML data: {0}")]
    ParseYaml(#[from] serde_yaml::Error),
    #[error("Failed to parse RSS data: {0}")]
    ParseRss(#[from] rss::Error),
    #[error("Polars error: {0}")]
    Polars(#[from] PolarsError),
    #[error("Version not found")]
    NotFound,
    #[error("Failed to parse version string")]
    VersionParse(#[from] semver::Error),
    #[error("GitHub API rate limited: {0}")]
    RateLimited(String),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(serde::Deserialize, Debug)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub prerelease: bool,
    pub draft: bool,
    pub html_url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GitHubTag {
    pub name: String,
}

#[async_trait]
pub trait Collector: Send + Sync {
    fn name(&self) -> &'static str;
    async fn collect(&self) -> Result<DataFrame, Error>;
}

pub use apache::ApacheCollector;
pub use kong::KongCollector;
