use crate::{Collector, Error, product_cycles_to_dataframe};
use async_trait::async_trait;
use polars::prelude::DataFrame;
use regex::Regex;
use semver::Version;
use std::collections::HashSet;
use versionwatch_core::domain::product_cycle::ProductCycle;

#[derive(serde::Deserialize, Debug)]
struct GitHubTag {
    name: String,
}

#[derive(serde::Deserialize, Debug)]
struct DockerHubTag {
    name: String,
}

#[derive(serde::Deserialize, Debug)]
struct DockerHubResponse {
    results: Vec<DockerHubTag>,
}

#[derive(Debug)]
pub struct KongCollector {
    name: String,
    github_token: Option<String>,
}

impl KongCollector {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            github_token: std::env::var("GITHUB_TOKEN").ok(),
        }
    }

    async fn fetch_github_tags(&self) -> Result<Vec<GitHubTag>, Error> {
        let url = "https://api.github.com/repos/Kong/kong/tags?per_page=100";
        let client = reqwest::Client::new();

        let mut request = client
            .get(url)
            .header("User-Agent", "versionwatch-collector")
            .header("Accept", "application/vnd.github.v3+json");

        if let Some(token) = &self.github_token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return match response.status() {
                reqwest::StatusCode::FORBIDDEN | reqwest::StatusCode::TOO_MANY_REQUESTS => {
                    println!("DEBUG: GitHub API rate limited for Kong, trying Docker Hub");
                    self.fetch_docker_tags().await
                }
                other => Err(Error::Other(anyhow::anyhow!(
                    "GitHub API returned unexpected status: {}",
                    other
                ))),
            };
        }

        let tags: Vec<GitHubTag> = response.json().await?;
        Ok(tags)
    }

    async fn fetch_docker_tags(&self) -> Result<Vec<GitHubTag>, Error> {
        let url = "https://hub.docker.com/v2/repositories/library/kong/tags/?page_size=500";
        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .header("User-Agent", "versionwatch-collector")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::Other(anyhow::anyhow!(
                "Docker Hub API returned status: {}",
                response.status()
            )));
        }

        let docker_response: DockerHubResponse = response.json().await?;

        // Convert DockerHubTag to GitHubTag format for consistency
        let tags = docker_response
            .results
            .into_iter()
            .map(|tag| GitHubTag { name: tag.name })
            .collect();

        Ok(tags)
    }
}

#[async_trait]
impl Collector for KongCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let tags = self.fetch_github_tags().await?;

        // Regex to match Kong version tags like "3.5.0", "3.4.3", etc.
        let re = Regex::new(r"^(\d+\.\d+\.\d+)(?:-.*)?$").unwrap();

        let mut seen_versions = HashSet::new();

        let cycles: Vec<ProductCycle> = tags
            .into_iter()
            .filter_map(|tag| {
                if let Some(captures) = re.captures(&tag.name) {
                    let version_str = captures.get(1)?.as_str();

                    if let Ok(version) = Version::parse(version_str) {
                        // Skip very old versions to keep the list manageable
                        if version.major < 2 {
                            return None;
                        }

                        // Deduplicate versions
                        if seen_versions.contains(&version) {
                            return None;
                        }
                        seen_versions.insert(version.clone());

                        Some(ProductCycle {
                            name: version.to_string(),
                            release_date: None, // Docker Hub doesn't provide release dates
                            eol_date: None,
                            lts: false,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        if cycles.is_empty() {
            return Err(Error::NotFound);
        }

        product_cycles_to_dataframe(cycles).map_err(Error::from)
    }
}
