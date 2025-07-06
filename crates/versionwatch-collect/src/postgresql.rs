use crate::{Collector, Error, ProductCycle, product_cycles_to_dataframe};
use async_trait::async_trait;
use chrono::NaiveDate;
use polars::prelude::DataFrame;
use scraper::{Html, Selector};

const POSTGRESQL_VERSIONING_URL: &str = "https://www.postgresql.org/support/versioning/";

#[derive(Debug)]
pub struct PostgresqlCollector {
    name: String,
}

impl PostgresqlCollector {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl Collector for PostgresqlCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let response = reqwest::get(POSTGRESQL_VERSIONING_URL)
            .await?
            .text()
            .await?;
        let document = Html::parse_document(&response);

        let table_selector = Selector::parse("table").unwrap();
        let row_selector = Selector::parse("tbody > tr").unwrap();
        let cell_selector = Selector::parse("td").unwrap();

        let mut cycles = Vec::new();

        if let Some(table) = document.select(&table_selector).next() {
            for row in table.select(&row_selector) {
                let cells: Vec<_> = row.select(&cell_selector).map(|c| c.inner_html()).collect();
                if cells.len() >= 4 {
                    let version_str = cells[0].trim();
                    let first_release_str = cells[2].trim();
                    let final_release_str = cells[3].trim();

                    let release_date =
                        NaiveDate::parse_from_str(first_release_str, "%B %e, %Y").ok();
                    let eol_date = NaiveDate::parse_from_str(final_release_str, "%B %e, %Y").ok();

                    cycles.push(ProductCycle {
                        name: version_str.to_string(),
                        release_date,
                        eol_date,
                        lts: false, // PostgreSQL doesn't have an official LTS concept in the same way other projects do
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
