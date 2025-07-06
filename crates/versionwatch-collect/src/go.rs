use crate::{Collector, Error, ProductCycle, product_cycles_to_dataframe};
use async_trait::async_trait;
use chrono::NaiveDate;
use polars::prelude::DataFrame;
use regex::Regex;
use std::collections::HashMap;

const GO_HISTORY_URL: &str = "https://go.dev/doc/devel/release";

pub struct GoCollector {
    name: String,
}

impl GoCollector {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Collector for GoCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let history_html = reqwest::get(GO_HISTORY_URL).await?.text().await?;
        let document = scraper::Html::parse_document(&history_html);
        let text = document.root_element().text().collect::<String>();

        let re = Regex::new(r"go(\d+\.\d+(?:\.\d+)?(?:rc\d+)?)\s+\(released\s+([\d-]+)\)").unwrap();
        let mut release_dates = HashMap::new();
        let mut major_release_dates = HashMap::new();

        for cap in re.captures_iter(&text) {
            let version_str = cap.get(1).unwrap().as_str();
            let date_str = cap.get(2).unwrap().as_str();

            if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                if version_str.ends_with(".0") {
                    if let Some(major_version) = version_str.rsplitn(2, '.').last() {
                        major_release_dates.insert(major_version.to_string(), date);
                    }
                }
                release_dates.insert(version_str.to_string(), date);
            }
        }

        let cycles = release_dates
            .into_iter()
            .filter(|(version, _)| !version.contains("rc"))
            .map(|(version_str, release_date)| {
                let parts: Vec<_> = version_str.split('.').collect();
                let eol_date = if parts.len() >= 2 {
                    let major_part = parts[0];
                    let minor_part = parts[1];
                    minor_part.parse::<u32>().ok().and_then(|minor_num| {
                        let eol_major_key = format!("{}.{}", major_part, minor_num + 2);
                        major_release_dates.get(&eol_major_key).cloned()
                    })
                } else {
                    None
                };

                ProductCycle {
                    name: version_str,
                    release_date: Some(release_date),
                    eol_date,
                    lts: false,
                }
            })
            .collect::<Vec<_>>();

        if cycles.is_empty() {
            return Err(Error::NotFound);
        }

        product_cycles_to_dataframe(cycles).map_err(Error::from)
    }
}
