use crate::{Collector, Error};
use async_trait::async_trait;
use polars::prelude::*;
use serde::Deserialize;

const GO_RELEASES_URL: &str = "https://go.dev/dl/?mode=json";

#[derive(Debug, Deserialize)]
struct GoRelease {
    version: String,
    stable: bool,
}

pub struct GoCollector;

#[async_trait]
impl Collector for GoCollector {
    fn name(&self) -> &'static str {
        "go"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let releases: Vec<GoRelease> = reqwest::get(GO_RELEASES_URL).await?.json().await?;

        let latest_stable = releases.iter().find(|r| r.stable).ok_or(Error::NotFound)?;

        let version_str = latest_stable.version.trim_start_matches("go").to_string();

        let df = df!(
            "name" => &["go"],
            "current_version" => &[""],
            "latest_version" => &[version_str],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[None::<String>],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }
}
