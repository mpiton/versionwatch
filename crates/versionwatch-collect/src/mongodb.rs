use crate::{Collector, Error, ProductCycle, product_cycles_to_dataframe};
use async_trait::async_trait;
use chrono::NaiveDate;
use polars::prelude::DataFrame;
use scraper::{Element, Html, Selector};

const MONGODB_LIFECYCLE_URL: &str = "https://www.mongodb.com/legal/support-policy/lifecycles";

#[derive(Debug)]
pub struct MongoDbCollector {
    name: String,
}

impl MongoDbCollector {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Collector for MongoDbCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let response = reqwest::get(MONGODB_LIFECYCLE_URL).await?.text().await?;
        let document = Html::parse_document(&response);

        let h3_selector = Selector::parse("h3").unwrap();
        let tr_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        let mut server_table = None;
        for h3 in document.select(&h3_selector) {
            if h3.text().collect::<String>().trim() == "MongoDB Server" {
                if let Some(table) = h3.next_sibling_element() {
                    server_table = Some(table);
                    break;
                }
            }
        }

        let table = server_table.ok_or(Error::NotFound)?;
        let mut cycles = Vec::new();

        for row in table.select(&tr_selector).skip(1) {
            // Skip header row
            let cells: Vec<_> = row
                .select(&td_selector)
                .map(|c| c.text().collect::<String>())
                .collect();
            if cells.len() >= 3 {
                let version = cells[0].trim().replace("MongoDB ", "");
                let eol_str = cells[2].trim();

                if let Ok(eol_date) = NaiveDate::parse_from_str(eol_str, "%B %d, %Y") {
                    cycles.push(ProductCycle {
                        name: version,
                        release_date: None,
                        eol_date: Some(eol_date),
                        lts: false, // Info not available on this page
                    });
                }
            }
        }

        if cycles.is_empty() {
            return Err(Error::NotFound);
        }

        product_cycles_to_dataframe(cycles).map_err(Error::from)
    }
}
