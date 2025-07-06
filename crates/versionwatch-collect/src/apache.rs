use crate::{Collector, Error, ProductCycle, product_cycles_to_dataframe};
use anyhow::anyhow;
use async_trait::async_trait;
use polars::prelude::DataFrame;
use regex::Regex;
use reqwest::StatusCode;
use std::collections::BTreeSet;

pub struct ApacheCollector;

impl ApacheCollector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ApacheCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Collector for ApacheCollector {
    fn name(&self) -> &str {
        "apache"
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let client = reqwest::Client::new();
        let mut versions = BTreeSet::new();

        // 1. Récupérer les versions actuelles depuis la page principale
        let current_url = "https://downloads.apache.org/httpd/";
        let response = client
            .get(current_url)
            .send()
            .await
            .map_err(|e| Error::Other(anyhow!("Failed to fetch Apache download page: {e}")))?;

        if response.status() != StatusCode::OK {
            return Err(Error::Other(anyhow!(
                "Apache download page returned status {}",
                response.status()
            )));
        }

        let html = response
            .text()
            .await
            .map_err(|e| Error::Other(anyhow!("Failed to read Apache download page: {e}")))?;

        // Pattern pour les versions actuelles: httpd-2.4.63.tar.gz
        let current_regex = Regex::new(r"httpd-(\d+\.\d+\.\d+)\.tar\.gz")
            .map_err(|e| Error::Other(anyhow!("Invalid regex pattern: {e}")))?;

        for cap in current_regex.captures_iter(&html) {
            let version = cap[1].to_string();
            versions.insert(version);
        }

        // 2. Récupérer les versions historiques depuis l'archive
        let archive_url = "http://archive.apache.org/dist/httpd/";
        let response = client
            .get(archive_url)
            .send()
            .await
            .map_err(|e| Error::Other(anyhow!("Failed to fetch Apache archive page: {e}")))?;

        if response.status() != StatusCode::OK {
            return Err(Error::Other(anyhow!(
                "Apache archive page returned status {}",
                response.status()
            )));
        }

        let html = response
            .text()
            .await
            .map_err(|e| Error::Other(anyhow!("Failed to read Apache archive page: {e}")))?;

        // Patterns pour les versions historiques
        let patterns = vec![
            // Apache 1.3.x: apache_1.3.42.tar.gz
            r"apache_(\d+\.\d+\.\d+)\.tar\.gz",
            // Apache 2.0.x: httpd-2.0.65.tar.gz
            r"httpd-(\d+\.\d+\.\d+)\.tar\.gz",
            // Apache 2.2.x: httpd-2.2.34.tar.gz
            r"httpd-(\d+\.\d+\.\d+)\.tar\.gz",
            // Apache 2.4.x: httpd-2.4.63.tar.gz
            r"httpd-(\d+\.\d+\.\d+)\.tar\.gz",
        ];

        for pattern in patterns {
            let regex = Regex::new(pattern)
                .map_err(|e| Error::Other(anyhow!("Invalid regex pattern: {e}")))?;

            for cap in regex.captures_iter(&html) {
                let version = cap[1].to_string();
                versions.insert(version);
            }
        }

        // 3. Convertir les versions en ProductCycle
        let mut cycles = Vec::new();
        for version in versions {
            let cycle = ProductCycle {
                name: version.clone(),
                release_date: None,
                eol_date: None,
                lts: false,
            };
            cycles.push(cycle);
        }

        // 4. Convertir en DataFrame
        product_cycles_to_dataframe(cycles).map_err(Error::from)
    }
}
