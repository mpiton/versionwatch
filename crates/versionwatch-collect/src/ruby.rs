use crate::{Collector, Error};
use async_trait::async_trait;
use polars::prelude::*;
use semver::Version;
use serde::Deserialize;

const RUBY_RELEASES_URL: &str =
    "https://raw.githubusercontent.com/ruby/www.ruby-lang.org/master/_data/releases.yml";

#[derive(Debug, Deserialize)]
struct RubyRelease {
    version: String,
    #[serde(default)]
    link: Option<String>,
}

pub struct RubyCollector;

#[async_trait]
impl Collector for RubyCollector {
    fn name(&self) -> &'static str {
        "ruby"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let response_text = reqwest::get(RUBY_RELEASES_URL).await?.text().await?;
        let releases: Vec<RubyRelease> = serde_yaml::from_str(&response_text)?;

        let latest_stable_release = releases
            .iter()
            .filter_map(|r| {
                Version::parse(&r.version)
                    .ok()
                    .filter(|v| v.pre.is_empty())
                    .map(|_v| r)
            })
            .max_by_key(|r| Version::parse(&r.version).unwrap_or_else(|_| Version::new(0, 0, 0)))
            .ok_or(Error::NotFound)?;

        let release_notes_url = latest_stable_release
            .link
            .as_ref()
            .map(|link| format!("https://www.ruby-lang.org{link}"));

        let df = df!(
            "name" => &["ruby"],
            "current_version" => &[""],
            "latest_version" => &[latest_stable_release.version.clone()],
            "latest_lts_version" => &[None::<String>],
            "is_lts" => &[false],
            "eol_date" => &[None::<i64>],
            "release_notes_url" => &[release_notes_url],
            "cve_count" => &[0_i32],
        )?;

        Ok(df)
    }
}
