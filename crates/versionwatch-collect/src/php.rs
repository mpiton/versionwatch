use crate::{Collector, Error, ProductCycle, product_cycles_to_dataframe};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use polars::prelude::DataFrame;
use scraper::{Element, Html, Selector};
use std::collections::HashMap;
use tracing::instrument;

#[derive(Debug)]
struct Version {
    name: String,
    date: NaiveDate,
}

#[instrument(err)]
async fn get_eol_dates() -> Result<HashMap<String, NaiveDate>> {
    let response = reqwest::get("https://www.php.net/supported-versions.php").await?;
    let eol_html = response.text().await?;
    let document = Html::parse_document(&eol_html);
    let row_selector = Selector::parse("table.standard tbody tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();
    let mut eol_dates = HashMap::new();

    for row in document.select(&row_selector) {
        let mut cells = row.select(&cell_selector);
        if let (Some(version_cell), Some(_), Some(eol_cell)) =
            (cells.next(), cells.nth(1), cells.next())
        {
            let version_str = version_cell.text().collect::<String>();
            let eol_str = eol_cell.text().collect::<String>();

            if let Ok(date) = NaiveDate::parse_from_str(eol_str.trim(), "%d %b %Y") {
                eol_dates.insert(version_str.trim().to_string(), date);
            }
        }
    }

    Ok(eol_dates)
}

#[instrument(err)]
async fn get_versions() -> Result<Vec<Version>> {
    let client = reqwest::Client::builder()
        .user_agent("versionwatch/0.1.0")
        .build()?;
    let response = client
        .get("https://www.php.net/releases/index.php")
        .send()
        .await?;
    let text = response.text().await?;
    let document = Html::parse_document(&text);

    let h2_selector = Selector::parse("h2").unwrap();
    let li_selector = Selector::parse("li").unwrap();

    let mut versions = Vec::new();

    for h2 in document.select(&h2_selector) {
        let version_str = h2.text().collect::<String>();
        if version_str.contains("x") {
            continue;
        }

        if let Some(ul) = h2.next_sibling_element() {
            for li in ul.select(&li_selector) {
                let li_text = li.text().collect::<String>();
                if li_text.starts_with("Released:") {
                    let date_str = li_text.replace("Released:", "").trim().to_string();
                    if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%d %b %Y") {
                        versions.push(Version {
                            name: version_str.clone(),
                            date,
                        });
                        break;
                    }
                }
            }
        }
    }

    Ok(versions)
}

pub struct PhpCollector {
    name: String,
}

impl PhpCollector {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Collector for PhpCollector {
    fn name(&self) -> &str {
        &self.name
    }

    #[instrument(err, skip(self))]
    async fn collect(&self) -> Result<DataFrame, Error> {
        let eol_dates = get_eol_dates().await?;
        let versions = get_versions().await?;

        let cycles: Vec<ProductCycle> = versions
            .into_iter()
            .map(|v| {
                let major_version = v.name.split('.').take(2).collect::<Vec<_>>().join(".");

                ProductCycle {
                    name: v.name.clone(),
                    release_date: Some(v.date),
                    eol_date: eol_dates.get(&major_version).cloned(),
                    lts: false,
                }
            })
            .collect();

        if cycles.is_empty() {
            return Err(Error::NotFound);
        }

        product_cycles_to_dataframe(cycles).map_err(Error::from)
    }
}
