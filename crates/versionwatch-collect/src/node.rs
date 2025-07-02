use super::{Collector, Error};
use async_trait::async_trait;
use polars::prelude::*;
use serde::Deserialize;

const NODEJS_RELEASES_URL: &str = "https://nodejs.org/dist/index.json";

#[derive(Debug, Deserialize)]
struct NodeVersion {
    version: String,
    lts: serde_json::Value, // Can be false (bool) or a string
}

pub struct NodeCollector;

#[async_trait]
impl Collector for NodeCollector {
    fn name(&self) -> &'static str {
        "node"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let response: Vec<NodeVersion> = reqwest::get(NODEJS_RELEASES_URL).await?.json().await?;

        let latest_version = response.first().ok_or(Error::NotFound)?;

        let latest_lts = response
            .iter()
            .find(|v| v.lts.is_string())
            .ok_or(Error::NotFound)?;

        let latest_version_str = latest_version.version.trim_start_matches('v').to_string();
        let latest_lts_version_str = Some(latest_lts.version.trim_start_matches('v').to_string());
        let is_lts = latest_version.version == latest_lts.version;
        let release_notes_url = Some(format!(
            "https://nodejs.org/en/blog/release/{}",
            latest_version.version
        ));

        let df = df!(
            "name" => &["node"],
            "current_version" => &[""],
            "latest_version" => &[latest_version_str],
            "latest_lts_version" => &[latest_lts_version_str],
            "is_lts" => &[is_lts],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[release_notes_url],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }
}
