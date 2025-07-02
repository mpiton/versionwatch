use super::{Collector, Error};
use async_trait::async_trait;
use polars::prelude::*;
use regex::Regex;
use reqwest::StatusCode;
use std::collections::BTreeSet;
use std::collections::HashMap;

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

        // Group by (major, minor) and keep the highest patch for each branch
        let mut latest_per_branch: HashMap<(u32, u32), u32> = HashMap::new();
        for (major, minor, patch) in &versions {
            let entry = latest_per_branch.entry((*major, *minor)).or_insert(*patch);
            if *patch > *entry {
                *entry = *patch;
            }
        }

        // Build DataFrame rows for each branch
        let mut names = Vec::new();
        let mut current_versions = Vec::new();
        let mut latest_versions = Vec::new();
        let mut latest_lts_versions = Vec::new();
        let mut is_lts = Vec::new();
        let mut eol_dates = Vec::new();
        let mut release_notes_urls = Vec::new();
        let mut cve_counts = Vec::new();

        for ((major, minor), patch) in latest_per_branch.iter() {
            names.push("apache");
            current_versions.push(None::<String>);
            latest_versions.push(format!("{major}.{minor}.{patch}"));
            latest_lts_versions.push(None::<String>);
            is_lts.push(false);
            eol_dates.push(None::<i64>);
            release_notes_urls.push(Some(format!("https://downloads.apache.org/httpd/CHANGES_{major}.{minor}")));
            cve_counts.push(0_i32);
        }

        let df = polars::df!(
            "name" => &names,
            "current_version" => &current_versions,
            "latest_version" => &latest_versions,
            "latest_lts_version" => &latest_lts_versions,
            "is_lts" => &is_lts,
            "eol_date" => &eol_dates,
            "release_notes_url" => &release_notes_urls,
            "cve_count" => &cve_counts,
        )
        .map_err(|e| Error::Other(e.to_string()))?;
        Ok(df)
    }
}
