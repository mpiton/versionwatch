use super::{Collector, Error};
use anyhow::anyhow;
use async_trait::async_trait;
use polars::prelude::*;
use regex::Regex;
use reqwest::StatusCode;
use std::collections::BTreeSet;

pub struct ApacheCollector;

impl ApacheCollector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ApacheCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Collector for ApacheCollector {
    fn name(&self) -> &'static str {
        "apache"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let url = "https://downloads.apache.org/httpd/";
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("User-Agent", "versionwatch-collector")
            .send()
            .await
            .map_err(|e| Error::Other(format!("Failed to fetch Apache download page: {e}")))?;

        if response.status() != StatusCode::OK {
            return Err(Error::Other(format!(
                "Apache download page returned status {}",
                response.status()
            )));
        }

        let body = response
            .text()
            .await
            .map_err(|e| Error::Other(format!("Failed to read Apache download page: {e}")))?;

        // Regex: only match httpd-X.Y.Z.tar.gz (no extra dot or dash after patch)
        let re = Regex::new(r"httpd-(\d+)\.(\d+)\.(\d+)\.tar\.gz")
            .map_err(|e| Error::Other(format!("Invalid regex pattern: {e}")))?;
        let mut versions = BTreeSet::new();
        for cap in re.captures_iter(&body) {
            let major: u32 = cap[1]
                .parse()
                .map_err(|_| Error::Other(format!("Invalid major version: {}", &cap[1])))?;
            let minor: u32 = cap[2]
                .parse()
                .map_err(|_| Error::Other(format!("Invalid minor version: {}", &cap[2])))?;
            let patch: u32 = cap[3]
                .parse()
                .map_err(|_| Error::Other(format!("Invalid patch version: {}", &cap[3])))?;
            // Only consider 2.x.x and above (ignore legacy 1.x)
            if major >= 2 {
                versions.insert((major, minor, patch));
            }
        }

        if versions.is_empty() {
            tracing::error!("No stable Apache versions found on download page");
            return Err(Error::Other("No stable Apache versions found".to_string()));
        }
        let latest_version = versions
            .iter()
            .max()
            .ok_or_else(|| anyhow!("No valid Apache versions found on the download page"))?;

        let df = polars::df!(
            "name" => &["apache"],
            "current_version" => &[None::<String>],
            "latest_version" => &[format!("{}.{}.{}", latest_version.0, latest_version.1, latest_version.2)],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[Some(format!("https://downloads.apache.org/httpd/CHANGES_{}.{}", latest_version.0, latest_version.1))],
            "cve_count" => &[0_i32],
        ).map_err(|e| Error::Other(e.to_string()))?;
        Ok(df)
    }
}
