use async_trait::async_trait;
use polars::prelude::*;
use reqwest::Client;
use serde::Deserialize;
use std::collections::BTreeMap;

use crate::Collector;

#[derive(Debug, Deserialize)]
struct PhpVersion {
    version: String,
}

pub struct PhpCollector {
    client: Client,
}

impl PhpCollector {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Default for PhpCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Collector for PhpCollector {
    fn name(&self) -> &str {
        "php"
    }

    async fn collect(&self) -> Result<DataFrame, crate::Error> {
        let response: BTreeMap<String, PhpVersion> = self
            .client
            .get("https://www.php.net/releases/index.php?json")
            .send()
            .await?
            .json()
            .await?;

        let versions: Vec<String> = response.into_values().map(|v| v.version).collect();

        let latest_version = versions
            .iter()
            .filter_map(|v| semver::Version::parse(v).ok())
            .max()
            .unwrap()
            .to_string();

        let df = df!(
            "name" => &["php"],
            "current_version" => &[""],
            "latest_version" => &[latest_version],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[None::<String>],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }
}
