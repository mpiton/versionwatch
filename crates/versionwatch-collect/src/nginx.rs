use crate::{Collector, Error, ProductCycle, product_cycles_to_dataframe};
use async_trait::async_trait;
use polars::prelude::DataFrame;
use serde::Deserialize;

const NGINX_RELEASES_URL: &str = "https://api.github.com/repos/nginx/nginx/tags";

#[derive(Debug, Deserialize)]
struct GitHubTag {
    name: String,
}

pub struct NginxCollector {
    name: String,
    github_token: Option<String>,
}

impl NginxCollector {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            github_token: std::env::var("GITHUB_TOKEN").ok(),
        }
    }
}

#[async_trait]
impl Collector for NginxCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let client = reqwest::Client::new();

        // Récupérer les tags depuis GitHub
        let mut request = client
            .get(NGINX_RELEASES_URL)
            .header("User-Agent", "VersionWatch/1.0")
            .header("Accept", "application/vnd.github.v3+json");

        // Ajouter le token GitHub si disponible
        if let Some(token) = &self.github_token {
            request = request.bearer_auth(token);
        }

        let response = request
            .send()
            .await
            .map_err(|e| Error::Other(anyhow::anyhow!("Failed to fetch Nginx releases: {e}")))?;

        let tags: Vec<GitHubTag> = response
            .json()
            .await
            .map_err(|e| Error::Other(anyhow::anyhow!("Failed to parse Nginx releases: {e}")))?;

        // Convertir en ProductCycle
        let mut cycles = Vec::new();
        for tag in tags {
            // Nettoyer le nom de version (enlever le préfixe 'release-' si présent)
            let version = tag.name.strip_prefix("release-").unwrap_or(&tag.name);

            // Ne garder que les versions qui ressemblent à des versions Nginx (x.y.z)
            if version.matches('.').count() >= 1
                && version.chars().next().unwrap_or('0').is_ascii_digit()
            {
                let cycle = ProductCycle {
                    name: version.to_string(),
                    release_date: None,
                    eol_date: None,
                    lts: false,
                };
                cycles.push(cycle);
            }
        }

        // Convertir vers DataFrame
        product_cycles_to_dataframe(cycles).map_err(Error::from)
    }
}
