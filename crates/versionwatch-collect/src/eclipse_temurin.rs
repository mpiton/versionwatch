use crate::{Collector, Error};
use async_trait::async_trait;
use polars::prelude::*;
use serde::Deserialize;

const ECLIPSE_TEMURIN_RELEASES_URL: &str = "https://api.adoptium.net/v3/info/release_versions?architecture=x64&page=0&page_size=20&release_type=ga&vendor=eclipse";

#[derive(Debug, Deserialize)]
struct EclipseTemurinVersions {
    versions: Vec<VersionInfo>,
}

#[derive(Debug, Deserialize)]
struct VersionInfo {
    major: u32,
    #[serde(default)]
    optional: Option<String>,
    #[serde(rename = "semver")]
    semver_str: String,
}

pub struct EclipseTemurinCollector;

#[async_trait]
impl Collector for EclipseTemurinCollector {
    fn name(&self) -> &str {
        "eclipse-temurin"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let response: EclipseTemurinVersions = reqwest::get(ECLIPSE_TEMURIN_RELEASES_URL)
            .await?
            .json()
            .await?;

        let latest_stable = response.versions.first().ok_or(Error::NotFound)?;
        let latest_lts = response
            .versions
            .iter()
            .find(|v| v.optional.as_deref() == Some("LTS"))
            .ok_or(Error::NotFound)?;

        let release_notes_url = format!(
            "https://github.com/adoptium/temurin{}-binaries/releases/tag/{}",
            latest_stable.major, latest_stable.semver_str
        );

        let df = df!(
            "name" => &["eclipse-temurin"],
            "current_version" => &[""],
            "latest_version" => &[latest_stable.semver_str.clone()],
            "latest_lts_version" => &[Some(latest_lts.semver_str.clone())],
            "is_lts" => &[latest_stable.optional.as_deref() == Some("LTS")],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[Some(release_notes_url)],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }
}
