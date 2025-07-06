use async_trait::async_trait;
use chrono::NaiveDate;
use polars::prelude::*;
use thiserror::Error;
use versionwatch_core::domain::product_cycle::ProductCycle;

pub mod apache;
pub mod caddy;
pub mod docker;
pub mod eclipse_temurin;
pub mod github;
pub mod go;
pub mod kong;
pub mod kotlin;
pub mod mongodb;
pub mod mysql;
pub mod nginx;
pub mod node;
pub mod perl;
pub mod php;
pub mod postgresql;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod scala;
pub mod swift;

#[derive(Debug, Error)]
pub enum Error {
    #[error("collector not found")]
    NotFound,
    #[error("rate limited: {0}")]
    RateLimited(String),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error(transparent)]
    SemVer(#[from] semver::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Rss(#[from] rss::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("Invalid token")]
    InvalidToken,
    #[error(transparent)]
    Polars(#[from] PolarsError),
}

#[derive(serde::Deserialize, Debug)]
pub struct GitHubRelease {
    #[serde(default)]
    pub name: String,
    pub tag_name: String,
    pub prerelease: bool,
    pub draft: bool,
    pub html_url: String,
    pub published_at: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GitHubTag {
    pub name: String,
}

/// Trait for collecting software version data into Polars DataFrames
///
/// This trait defines the interface for all version collectors in the VersionWatch ETL system.
/// Each collector fetches version information from various sources and returns it as an optimized
/// Polars DataFrame for efficient processing and analysis.
#[async_trait]
pub trait Collector: Send + Sync {
    /// Returns the name of the software this collector tracks
    fn name(&self) -> &str;

    /// Collects version data and returns it as a Polars DataFrame
    ///
    /// The DataFrame should contain the following columns:
    /// - `name`: String - Version name (e.g., "1.21.0")
    /// - `release_date`: Option<Date> - Release date if available
    /// - `eol_date`: Option<Date> - End-of-life date if available  
    /// - `lts`: Boolean - Whether this is an LTS version
    async fn collect(&self) -> Result<DataFrame, Error>;
}

/// Helper function to convert Vec<ProductCycle> to Polars DataFrame
///
/// This utility function helps with the migration from the old Vec<ProductCycle> approach
/// to the new Polars DataFrame approach. It creates a properly typed DataFrame with the
/// standard schema expected by the VersionWatch ETL system.
pub fn product_cycles_to_dataframe(cycles: Vec<ProductCycle>) -> PolarsResult<DataFrame> {
    let names: Vec<String> = cycles.iter().map(|c| c.name.clone()).collect();
    let release_dates: Vec<Option<i32>> = cycles
        .iter()
        .map(|c| {
            c.release_date.map(|d| {
                // Convert to days since epoch (1970-01-01)
                let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
                (d - epoch).num_days() as i32
            })
        })
        .collect();
    let eol_dates: Vec<Option<i32>> = cycles
        .iter()
        .map(|c| {
            c.eol_date.map(|d| {
                // Convert to days since epoch (1970-01-01)
                let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
                (d - epoch).num_days() as i32
            })
        })
        .collect();
    let lts_flags: Vec<bool> = cycles.iter().map(|c| c.lts).collect();

    df!(
        "name" => names,
        "release_date" => release_dates,
        "eol_date" => eol_dates,
        "lts" => lts_flags,
    )
}

/// Helper function to convert Polars DataFrame to Vec<ProductCycle>
///
/// This utility function helps with backward compatibility during the migration.
/// It extracts ProductCycle structs from a properly formatted DataFrame.
pub fn dataframe_to_product_cycles(df: &DataFrame) -> PolarsResult<Vec<ProductCycle>> {
    let names = df
        .column("name")?
        .str()?
        .into_no_null_iter()
        .collect::<Vec<_>>();
    let release_dates = df.column("release_date")?.i32()?;
    let eol_dates = df.column("eol_date")?.i32()?;
    let lts_flags = df
        .column("lts")?
        .bool()?
        .into_no_null_iter()
        .collect::<Vec<_>>();

    let mut cycles = Vec::new();
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

    for i in 0..df.height() {
        let release_date = release_dates
            .get(i)
            .map(|days| epoch + chrono::Duration::days(days as i64));
        let eol_date = eol_dates
            .get(i)
            .map(|days| epoch + chrono::Duration::days(days as i64));

        cycles.push(ProductCycle {
            name: names[i].to_string(),
            release_date,
            eol_date,
            lts: lts_flags[i],
        });
    }

    Ok(cycles)
}

pub use apache::ApacheCollector;
pub use caddy::CaddyCollector;
pub use docker::DockerCollector;
pub use eclipse_temurin::EclipseTemurinCollector;
pub use github::GitHubCollector;
pub use go::GoCollector;
pub use kong::KongCollector;
pub use kotlin::KotlinCollector;
pub use mongodb::MongoDbCollector;
pub use mysql::MySqlCollector;
pub use nginx::NginxCollector;
pub use node::NodeCollector;
pub use perl::PerlCollector;
pub use php::PhpCollector;
pub use postgresql::PostgresqlCollector;
pub use python::PythonCollector;
pub use ruby::RubyCollector;
pub use rust::RustCollector;
pub use scala::ScalaCollector;
pub use swift::SwiftCollector;
