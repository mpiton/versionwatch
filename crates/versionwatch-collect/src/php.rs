use crate::{Collector, Error};
use async_trait::async_trait;
use polars::prelude::*;
use serde::Deserialize;

const PHP_RELEASES_URL: &str = "https://www.php.net/releases/index.php?json";

#[derive(Debug, Deserialize)]
struct PhpVersion {
    version: String,
}

pub struct PhpCollector;

#[async_trait]
impl Collector for PhpCollector {
    fn name(&self) -> &'static str {
        "php"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let response: serde_json::Value = reqwest::get(PHP_RELEASES_URL).await?.json().await?;
        let versions = response
            .as_object()
            .ok_or(Error::NotFound)?
            .values()
            .filter_map(|v| serde_json::from_value::<PhpVersion>(v.clone()).ok());

        let latest_version = versions
            .filter_map(|v| semver::Version::parse(&v.version).ok())
            .max()
            .ok_or(Error::NotFound)?;

        let df = df!(
            "name" => &["php"],
            "current_version" => &[""],
            "latest_version" => &[latest_version.to_string()],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[None::<String>],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }
}
