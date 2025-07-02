use super::{Collector, Error};
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

        // Regex: httpd-2.4.59.tar.gz or httpd-2.4.59-rc1.tar.gz
        let re = Regex::new(r"httpd-(\d+)\.(\d+)\.(\d+)(?:-([a-zA-Z0-9]+))?\.tar\.gz")
            .map_err(|e| Error::Other(format!("Invalid regex pattern: {e}")))?;
        let mut versions = BTreeSet::new();
        for cap in re.captures_iter(&body) {
            // cap[4] is the optional suffix (rc1, alpha, beta, etc)
            if cap.get(4).is_some() {
                // Ignore pre-releases
                continue;
            }
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

        let latest = versions.iter().next_back();
        let latest_version = match latest {
            Some((major, minor, patch)) => format!("{major}.{minor}.{patch}"),
            None => {
                tracing::error!("No stable Apache version found on download page");
                return Err(Error::Other("No stable Apache version found".to_string()));
            }
        };

        let df = polars::df!(
            "name" => &["apache"],
            "current_version" => &[None::<String>],
            "latest_version" => &[latest_version],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[Some("https://downloads.apache.org/httpd/".to_string())],
            "cve_count" => &[0_i32],
        )
        .map_err(|e| Error::Other(e.to_string()))?;
        Ok(df)
    }
}
