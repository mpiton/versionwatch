use super::{Collector, Error, GitHubRelease, GitHubTag};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use polars::prelude::*;
use semver::Version;
use versionwatch_config::VersionCleaning;

/// A collector for software that publishes releases on GitHub.
pub struct GitHubCollector {
    name: String,
    repo: String,
    github_token: Option<String>,
    source: String,
    cleaning: VersionCleaning,
}

impl GitHubCollector {
    pub fn new(
        name: String,
        repo: String,
        github_token: Option<String>,
        source: String,
        cleaning: VersionCleaning,
    ) -> Self {
        Self {
            name,
            repo,
            github_token,
            source,
            cleaning,
        }
    }

    fn clean_version(&self, version_str: String) -> String {
        let mut version = version_str;
        if let Some(prefix) = &self.cleaning.trim_prefix {
            if let Some(trimmed) = version.strip_prefix(prefix) {
                version = trimmed.to_string();
            }
        }
        if let Some(suffix) = &self.cleaning.trim_suffix {
            if let Some(trimmed) = version.strip_suffix(suffix) {
                version = trimmed.to_string();
            }
        }
        version.trim_start_matches('v').to_string()
    }
}

#[async_trait]
impl Collector for GitHubCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        if self.source == "releases" {
            self.collect_from_releases().await
        } else if self.source == "tags" {
            self.collect_from_tags().await
        } else {
            Err(Error::Other(format!(
                "Unsupported GitHub source: {}",
                self.source
            )))
        }
    }
}

impl GitHubCollector {
    async fn collect_from_releases(&self) -> Result<DataFrame, Error> {
        let url = format!("https://api.github.com/repos/{}/releases", self.repo);
        let releases: Vec<GitHubRelease> = self.fetch(&url).await?;

        let latest_release = releases
            .into_iter()
            .find(|r| !r.prerelease && !r.draft)
            .ok_or(Error::NotFound)?;

        let version = self.clean_version(latest_release.tag_name.clone());

        let df = df!(
            "name" => &[self.name.as_str()],
            "current_version" => &[""],
            "latest_version" => &[version],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[Some(latest_release.html_url)],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }

    async fn collect_from_tags(&self) -> Result<DataFrame, Error> {
        let url = format!("https://api.github.com/repos/{}/tags", self.repo);
        let tags: Vec<GitHubTag> = self.fetch(&url).await?;

        if tags.is_empty() {
            return Err(Error::NotFound);
        }

        let latest_tag = tags
            .iter()
            .filter_map(|tag| {
                Version::parse(tag.name.trim_start_matches('v'))
                    .ok()
                    .filter(|v| v.pre.is_empty())
                    .map(|v| (v, tag))
            })
            .max_by(|(v1, _), (v2, _)| v1.cmp(v2))
            .map(|(_, tag)| tag)
            .ok_or_else(|| {
                Error::Other(
                    "No stable tags found. All tags appear to be pre-releases.".to_string(),
                )
            })?;

        let version_string = self.clean_version(latest_tag.name.clone());

        let release_notes_url = format!(
            "https://github.com/{}/releases/tag/{}",
            self.repo, latest_tag.name
        );

        let df = df!(
            "name" => &[self.name.as_str()],
            "current_version" => &[""],
            "latest_version" => &[version_string],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[Some(release_notes_url)],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }

    async fn fetch<T: serde::de::DeserializeOwned>(
        &self,
        initial_url: &str,
    ) -> Result<Vec<T>, Error> {
        let mut results = Vec::new();
        let mut next_url = Some(initial_url.to_string());
        let client = reqwest::Client::new();
        let mut page_count = 0;
        const MAX_PAGES: u8 = 10; // Limit pagination to avoid excessive API calls

        while let Some(url) = next_url {
            if page_count >= MAX_PAGES {
                tracing::warn!(
                    "Reached maximum page limit ({}) for {}. Results may be incomplete.",
                    MAX_PAGES,
                    self.repo
                );
                break;
            }
            page_count += 1;

            let mut request = client
                .get(&url)
                .header("User-Agent", "versionwatch-collector")
                .header("Accept", "application/vnd.github.v3+json");

            if let Some(token) = &self.github_token {
                request = request.bearer_auth(token);
            }

            let response = request.send().await?;

            if !response.status().is_success() {
                return match response.status() {
                    reqwest::StatusCode::FORBIDDEN => {
                        let error_msg = if self.github_token.is_some() {
                            "GitHub API returned 403 Forbidden. The token may be invalid or lack permissions."
                        } else {
                            "GitHub API returned 403 Forbidden. A GITHUB_TOKEN may be required for this repository."
                        };
                        Err(Error::Other(error_msg.to_string()))
                    }
                    reqwest::StatusCode::TOO_MANY_REQUESTS => {
                        let reset = response
                            .headers()
                            .get("x-ratelimit-reset")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|s| s.parse::<u64>().ok())
                            .unwrap_or(0);

                        let reset_time = DateTime::<Utc>::from_timestamp(reset as i64, 0)
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                            .unwrap_or_else(|| {
                                format!(
                                    "{} seconds from now",
                                    reset.saturating_sub(Utc::now().timestamp() as u64)
                                )
                            });

                        let wait_msg =
                            format!("GitHub API rate limit exceeded. Try again at {reset_time}.");
                        Err(Error::RateLimited(wait_msg))
                    }
                    other => Err(Error::Other(format!(
                        "GitHub API returned unexpected status: {other}"
                    ))),
                };
            }

            let link_header = response
                .headers()
                .get(reqwest::header::LINK)
                .and_then(|v| v.to_str().ok());
            next_url = parse_link_header(link_header);

            let mut page_results: Vec<T> = response.json().await?;
            results.append(&mut page_results);
        }

        Ok(results)
    }
}

fn parse_link_header(header: Option<&str>) -> Option<String> {
    header?.split(',').find_map(|part| {
        let mut segments = part.split(';');
        let url_part = segments.next()?;
        let rel_part = segments.next()?;
        if rel_part.trim() == "rel=\"next\"" {
            let url = url_part
                .trim()
                .trim_start_matches('<')
                .trim_end_matches('>');
            Some(url.to_string())
        } else {
            None
        }
    })
}
