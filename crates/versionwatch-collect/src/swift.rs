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
pub struct SwiftCollector {
    name: String,
    github_token: Option<String>,
}

impl SwiftCollector {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            github_token: std::env::var("GITHUB_TOKEN").ok(),
        }
    }

    async fn fetch_github_tags(&self) -> Result<Vec<GitHubTag>, Error> {
        let url = "https://api.github.com/repos/swiftlang/swift/tags?per_page=100";
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
                    println!("DEBUG: GitHub API rate limited for Swift, trying Docker Hub");
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
        let url = "https://hub.docker.com/v2/repositories/library/swift/tags/?page_size=500";
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

    fn process_tags(&self, tags: Vec<GitHubTag>) -> Vec<ProductCycle> {
        // Enhanced regex to match various Swift version formats
        let re = Regex::new(r"(?:swift-)?(\d+\.\d+(?:\.\d+)?)(?:-.*)?$").unwrap();
        let mut seen_versions = HashSet::new();

        tags.into_iter()
            .filter_map(|tag| {
                if let Some(captures) = re.captures(&tag.name) {
                    let version_str = captures.get(1)?.as_str();

                    // Handle versions without patch (e.g., "5.9" -> "5.9.0")
                    let normalized_version = if version_str.matches('.').count() == 1 {
                        format!("{version_str}.0")
                    } else {
                        version_str.to_string()
                    };

                    if let Ok(version) = Version::parse(&normalized_version) {
                        // Skip very old versions to keep the list manageable
                        if version.major < 5 {
                            return None;
                        }

                        // Deduplicate versions
                        if seen_versions.contains(&version) {
                            return None;
                        }
                        seen_versions.insert(version.clone());

                        Some(ProductCycle {
                            name: version.to_string(),
                            release_date: None,
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
            .collect()
    }
}

#[async_trait]
impl Collector for SwiftCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        // Try Docker Hub first, then fallback to GitHub
        match self.fetch_docker_tags().await {
            Ok(tags) => {
                println!("DEBUG: Swift Docker Hub succeeded with {} tags", tags.len());
                let cycles = self.process_tags(tags);
                if !cycles.is_empty() {
                    return product_cycles_to_dataframe(cycles).map_err(Error::from);
                }
            }
            Err(_) => {
                println!("DEBUG: Swift Docker Hub failed, trying GitHub");
            }
        }

        // Fallback to GitHub
        match self.fetch_github_tags().await {
            Ok(tags) => {
                println!("DEBUG: Swift GitHub succeeded with {} tags", tags.len());
                let cycles = self.process_tags(tags);
                if !cycles.is_empty() {
                    return product_cycles_to_dataframe(cycles).map_err(Error::from);
                }
            }
            Err(_) => {
                println!("DEBUG: Swift GitHub failed, using known versions");
            }
        }

        // Final fallback: known Swift versions
        let known_versions = vec![
            "6.0.3", "6.0.2", "6.0.1", "6.0.0", "5.10.1", "5.10.0", "5.9.2", "5.9.1", "5.9.0",
            "5.8.1", "5.8.0", "5.7.3", "5.7.2", "5.7.1", "5.7.0", "5.6.3", "5.6.2", "5.6.1",
            "5.6.0", "5.5.3", "5.5.2", "5.5.1", "5.5.0", "5.4.3", "5.4.2", "5.4.1", "5.4.0",
            "5.3.3", "5.3.2", "5.3.1", "5.3.0",
        ];

        let mut cycles = Vec::new();
        for version_str in known_versions {
            if let Ok(version) = Version::parse(version_str) {
                cycles.push(ProductCycle {
                    name: version.to_string(),
                    release_date: None,
                    eol_date: None,
                    lts: false,
                });
            }
        }

        println!("DEBUG: Swift using {} known versions", cycles.len());
        product_cycles_to_dataframe(cycles).map_err(Error::from)
    }
}
