use crate::{Collector, Error, ProductCycle, product_cycles_to_dataframe};
use async_trait::async_trait;
use chrono::NaiveDate;
use polars::prelude::DataFrame;
use serde::Deserialize;
use std::collections::HashMap;

const NODE_RELEASES_URL: &str = "https://nodejs.org/dist/index.json";
const NODE_SCHEDULE_URL: &str =
    "https://raw.githubusercontent.com/nodejs/release/main/schedule.json";

#[derive(Debug, Deserialize)]
struct NodeVersion {
    version: String,
    date: NaiveDate,
    lts: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct NodeSchedule {
    end: NaiveDate,
}

pub struct NodeCollector {
    name: String,
}

impl NodeCollector {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Collector for NodeCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let releases: Vec<NodeVersion> = reqwest::get(NODE_RELEASES_URL).await?.json().await?;
        let schedule: HashMap<String, NodeSchedule> =
            reqwest::get(NODE_SCHEDULE_URL).await?.json().await?;

        let eol_map: HashMap<String, NaiveDate> = schedule
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.end))
            .collect();

        let cycles = releases
            .into_iter()
            .map(|v| {
                let version_str = v.version.trim_start_matches('v');
                let major_version_key = format!("v{}", version_str.split('.').next().unwrap_or(""));
                let eol_date = eol_map.get(&major_version_key).cloned();

                ProductCycle {
                    name: version_str.to_string(),
                    release_date: Some(v.date),
                    eol_date,
                    lts: v.lts.is_string(),
                }
            })
            .collect();

        product_cycles_to_dataframe(cycles).map_err(Error::from)
    }
}

/*
#[async_trait]
impl Collector for NodeCollector {
    fn name(&self) -> &str {
        "node"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let response: Vec<NodeVersion> = reqwest::get(NODE_RELEASES_URL).await?.json().await?;

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
            "eol_date" => &[None::<NaiveDate>],
            "release_notes_url" => &[release_notes_url],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }
}
*/
