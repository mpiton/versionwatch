use super::Error;
use crate::{Collector, product_cycles_to_dataframe};
use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use polars::prelude::DataFrame;
use regex::Regex;
use semver::Version;
use serde::Deserialize;
use std::time::Duration;
use versionwatch_core::domain::product_cycle::ProductCycle;

#[derive(Debug, Clone)]
pub enum GitHubSource {
    Releases,
    Tags,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    published_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubTag {
    name: String,
    commit: CommitInfo,
}

#[derive(Debug, Deserialize)]
struct CommitInfo {
    url: String,
}

#[derive(Debug, Deserialize)]
struct GitHubCommit {
    commit: CommitDetails,
}

#[derive(Debug, Deserialize)]
struct CommitDetails {
    committer: Committer,
}

#[derive(Debug, Deserialize)]
struct Committer {
    date: chrono::DateTime<chrono::Utc>,
}

/// A collector for software that publishes releases on GitHub.
#[derive(Clone, Debug)]
pub struct GitHubCollector {
    name: String,
    repository: String,
    source: GitHubSource,
}

impl GitHubCollector {
    pub fn new(name: &str, repository: &str, source: GitHubSource) -> Self {
        Self {
            name: name.to_string(),
            repository: repository.to_string(),
            source,
        }
    }
}

#[async_trait]
impl Collector for GitHubCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        match self.source {
            GitHubSource::Releases => self.collect_from_releases().await,
            GitHubSource::Tags => self.collect_from_tags().await,
        }
    }
}

impl GitHubCollector {
    async fn collect_from_releases(&self) -> Result<DataFrame, Error> {
        let url = format!("https://api.github.com/repos/{}/releases", self.repository);
        let releases: Vec<GitHubRelease> = self.fetch(&url).await?;
        let re = Regex::new(r"(\d+[\._]\d+([\._]\d+)?)").unwrap();

        let mut cycles = Vec::new();
        for release in releases {
            if let Some(captures) = re.captures(&release.tag_name) {
                let version_str = captures.get(1).unwrap().as_str();
                let clean_version = version_str.replace('_', ".");

                if let Ok(version) = Version::parse(&clean_version) {
                    let release_date = release.published_at.and_then(|date_str: String| {
                        chrono::DateTime::parse_from_rfc3339(&date_str)
                            .map(|dt| dt.naive_utc().date())
                            .ok()
                    });

                    cycles.push(ProductCycle {
                        name: version.to_string(),
                        release_date,
                        eol_date: None,
                        lts: false,
                    });
                }
            }
        }

        if cycles.is_empty() {
            return Err(Error::NotFound);
        }

        product_cycles_to_dataframe(cycles).map_err(Error::from)
    }

    async fn collect_from_tags(&self) -> Result<DataFrame, Error> {
        let url = format!("https://api.github.com/repos/{}/tags", self.repository);
        let tags: Vec<GitHubTag> = self.fetch(&url).await?;
        let re = Regex::new(r"(\d+[\._]\d+([\._]\d+)?)").unwrap();

        let versions_with_urls: Vec<(String, String)> = tags
            .into_iter()
            .filter_map(|tag: GitHubTag| {
                re.captures(&tag.name).map(|caps| {
                    let version_str = caps.get(1).unwrap().as_str();
                    let clean_version = version_str.replace('_', ".");
                    (clean_version, tag.commit.url)
                })
            })
            .collect();

        let client = reqwest::Client::builder()
            .user_agent("versionwatch")
            .build()?;

        let cycles: Vec<ProductCycle> = stream::iter(versions_with_urls)
            .map(|(version, url)| {
                let client = client.clone();
                async move {
                    if let Ok(parsed_version) = Version::parse(&version) {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        let resp = client.get(&url).send().await;
                        let release_date = match resp {
                            Ok(r) => {
                                let commit: Result<GitHubCommit, _> = r.json().await;
                                commit
                                    .ok()
                                    .map(|c| c.commit.committer.date.naive_utc().date())
                            }
                            Err(_) => None,
                        };

                        Some(ProductCycle {
                            name: parsed_version.to_string(),
                            release_date,
                            eol_date: None,
                            lts: false,
                        })
                    } else {
                        None
                    }
                }
            })
            .buffer_unordered(10)
            .filter_map(|x| async { x })
            .collect()
            .await;

        if cycles.is_empty() {
            return Err(Error::NotFound);
        }

        product_cycles_to_dataframe(cycles).map_err(Error::from)
    }

    async fn fetch<T>(&self, url: &str) -> Result<Vec<T>, Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        const MAX_PAGES: usize = 10;
        let mut page = 1;
        let mut all_items = Vec::new();

        let client = reqwest::Client::builder()
            .user_agent("versionwatch")
            .build()?;

        loop {
            if page > MAX_PAGES {
                eprintln!(
                    "Reached maximum page limit ({}) for {}. Results may be incomplete.",
                    MAX_PAGES, self.repository
                );
                break;
            }

            let page_url = format!("{url}?page={page}&per_page=100");
            let request = client
                .get(&page_url)
                .header("Accept", "application/vnd.github.v3+json");

            let response = request.send().await?;

            if !response.status().is_success() {
                return match response.status() {
                    reqwest::StatusCode::FORBIDDEN => Err(Error::Other(anyhow::anyhow!(
                        "GitHub API returned 403 Forbidden. A GITHUB_TOKEN may be required for this repository."
                    ))),
                    reqwest::StatusCode::TOO_MANY_REQUESTS => Err(Error::RateLimited(
                        "GitHub API rate limit exceeded".to_string(),
                    )),
                    _ => Err(Error::Other(anyhow::anyhow!(
                        "GitHub API request failed with status: {}",
                        response.status()
                    ))),
                };
            }

            let items: Vec<T> = response.json().await?;

            if items.is_empty() {
                break;
            }

            all_items.extend(items);
            page += 1;
        }

        if all_items.is_empty() {
            return Err(Error::NotFound);
        }

        Ok(all_items)
    }
}
