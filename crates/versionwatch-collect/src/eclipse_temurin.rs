use crate::{Collector, Error, ProductCycle, product_cycles_to_dataframe};
use async_trait::async_trait;
use polars::prelude::DataFrame;
use serde::Deserialize;

use futures::stream::{self, StreamExt};
use std::time::Duration;

const ECLIPSE_TEMURIN_BASE_URL: &str = "https://api.adoptium.net/v3/info/release_versions?architecture=x64&release_type=ga&vendor=eclipse";

#[derive(Deserialize, Debug)]
struct GitHubCommit {
    commit: CommitDetails,
}

#[derive(Deserialize, Debug)]
struct CommitDetails {
    committer: Committer,
}

#[derive(Deserialize, Debug)]
struct Committer {
    date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
struct EclipseTemurinVersions {
    versions: Vec<VersionInfo>,
}

#[derive(Debug, Deserialize, Clone)]
struct VersionInfo {
    major: u32,
    #[serde(default)]
    optional: Option<String>,
    #[serde(rename = "semver")]
    semver_str: String,
}

pub struct EclipseTemurinCollector {
    name: String,
}

impl EclipseTemurinCollector {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Collector for EclipseTemurinCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let mut headers = reqwest::header::HeaderMap::new();

        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("token {token}"))
                    .map_err(|_| Error::InvalidToken)?,
            );
        }

        let client = reqwest::Client::builder()
            .user_agent("versionwatch")
            .default_headers(headers)
            .build()?;

        let mut all_versions: Vec<VersionInfo> = Vec::new();
        let mut page = 0;

        loop {
            let url = format!("{ECLIPSE_TEMURIN_BASE_URL}&page={page}&page_size=100");
            let response_text = reqwest::get(&url).await?.text().await?;

            if response_text.trim().is_empty() || response_text.trim() == "{}" {
                break;
            }

            let response: EclipseTemurinVersions = serde_json::from_str(&response_text)?;

            if response.versions.is_empty() {
                break;
            }

            all_versions.extend(response.versions);
            page += 1;
        }

        let cycles: Vec<ProductCycle> = stream::iter(all_versions)
            .map(|v| {
                let client = client.clone();
                async move {
                    let repo = format!("adoptium/temurin{}-binaries", v.major);
                    let tag = v.semver_str.clone();
                    let commit_url = format!("https://api.github.com/repos/{repo}/git/tags/{tag}");

                    tokio::time::sleep(Duration::from_millis(100)).await;
                    let resp = client.get(&commit_url).send().await;

                    let release_date = match resp {
                        Ok(r) => {
                            let commit: Result<GitHubCommit, _> = r.json().await;
                            commit
                                .ok()
                                .map(|c| c.commit.committer.date.naive_utc().date())
                        }
                        Err(_) => None,
                    };

                    ProductCycle {
                        name: v.semver_str,
                        release_date,
                        eol_date: None,
                        lts: v.optional.as_deref() == Some("LTS"),
                    }
                }
            })
            .buffer_unordered(10)
            .collect()
            .await;

        if cycles.is_empty() {
            return Err(Error::NotFound);
        }

        product_cycles_to_dataframe(cycles).map_err(Error::from)
    }
}

/*
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
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[latest_stable.optional.as_deref() == Some("LTS")],
            "eol_date" => &[None::<NaiveDate>],
            "release_notes_url" => &[Some(format!("https://github.com/adoptium/temurin{}-binaries/releases/tag/{}", latest_stable.major, latest_stable.semver_str))],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }
}
*/
