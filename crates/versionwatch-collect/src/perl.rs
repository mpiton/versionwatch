use crate::{Collector, Error};
use async_trait::async_trait;
use polars::prelude::*;
use serde::Deserialize;

const PERL_RELEASES_URL: &str = "https://fastapi.metacpan.org/release/perl";

#[derive(Debug, Deserialize)]
struct MetaCpanResponse {
    version: String,
}

pub struct PerlCollector;

#[async_trait]
impl Collector for PerlCollector {
    fn name(&self) -> &str {
        "perl"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let response: MetaCpanResponse = reqwest::get(PERL_RELEASES_URL).await?.json().await?;

        let version_parts: Vec<&str> = response.version.split('.').collect();
        let major = version_parts.first().unwrap_or(&"");
        let minor = version_parts
            .get(1)
            .map(|s| s.parse::<i32>().unwrap_or(0) / 1000)
            .unwrap_or(0);
        let patch = version_parts
            .get(1)
            .map(|s| s.parse::<i32>().unwrap_or(0) % 1000)
            .unwrap_or(0);

        let latest_version = format!("{major}.{minor}.{patch}");

        let df = df!(
            "name" => &["perl"],
            "current_version" => &[""],
            "latest_version" => &[latest_version],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[Some("https://metacpan.org/pod/perldelta".to_string())],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }
}
