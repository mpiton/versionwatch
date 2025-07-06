use crate::{Collector, Error, ProductCycle, product_cycles_to_dataframe};
use async_trait::async_trait;
use polars::prelude::DataFrame;
use serde::Deserialize;
use std::collections::HashMap;

const PERL_RELEASES_URL: &str =
    "https://fastapi.metacpan.org/v1/release/_search?q=distribution:perl&size=1000";

#[derive(Debug, Deserialize)]
struct MetaCpanSearchResponse {
    hits: MetaCpanHits,
}

#[derive(Debug, Deserialize)]
struct MetaCpanHits {
    hits: Vec<MetaCpanHit>,
}

#[derive(Debug, Deserialize)]
struct MetaCpanHit {
    #[serde(rename = "_source")]
    source: MetaCpanRelease,
}

#[derive(Debug, Deserialize)]
struct MetaCpanRelease {
    version: String,
    date: String,
    status: String,
    maturity: String,
}

pub struct PerlCollector;

#[async_trait]
impl Collector for PerlCollector {
    fn name(&self) -> &str {
        "perl"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let response: MetaCpanSearchResponse =
            reqwest::get(PERL_RELEASES_URL).await?.json().await?;

        let mut version_map = HashMap::new();

        for hit in response.hits.hits {
            let release = hit.source;

            // Only keep stable releases from CPAN (not development versions)
            if release.status != "cpan" || release.maturity != "released" {
                continue;
            }

            // Skip development versions (those with underscores)
            if release.version.contains("_") {
                continue;
            }

            // Parse version to ensure it's a valid Perl version (5.x.x format)
            let version_parts: Vec<&str> = release.version.split('.').collect();
            if version_parts.len() >= 2 {
                if let Ok(major) = version_parts[0].parse::<u32>() {
                    if major >= 5 {
                        // Parse release date
                        let release_date = chrono::DateTime::parse_from_rfc3339(&release.date)
                            .ok()
                            .map(|dt| dt.naive_utc().date());

                        version_map.insert(release.version.clone(), release_date);
                    }
                }
            }
        }

        let mut cycles: Vec<ProductCycle> = version_map
            .into_iter()
            .map(|(version, release_date)| ProductCycle {
                name: version,
                release_date,
                eol_date: None, // Perl doesn't have official EOL dates
                lts: false,     // Perl doesn't have LTS versions
            })
            .collect();

        // Sort by version number
        cycles.sort_by(|a, b| {
            let a_parts: Vec<u32> = a.name.split('.').filter_map(|s| s.parse().ok()).collect();
            let b_parts: Vec<u32> = b.name.split('.').filter_map(|s| s.parse().ok()).collect();
            a_parts.cmp(&b_parts)
        });

        if cycles.is_empty() {
            return Err(Error::NotFound);
        }

        product_cycles_to_dataframe(cycles).map_err(Error::from)
    }
}
