use crate::{Collector, Error, ProductCycle, product_cycles_to_dataframe};
use async_trait::async_trait;
use polars::prelude::DataFrame;
use serde::Deserialize;

const RUST_GITHUB_API: &str = "https://api.github.com/repos/rust-lang/rust/releases";

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    prerelease: bool,
}

pub struct RustCollector {
    name: String,
    github_token: Option<String>,
}

impl RustCollector {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            github_token: std::env::var("GITHUB_TOKEN").ok(),
        }
    }
}

#[async_trait]
impl Collector for RustCollector {
    fn name(&self) -> &str {
        &self.name
    }

    async fn collect(&self) -> Result<DataFrame, Error> {
        let client = reqwest::Client::new();

        // Récupérer les releases depuis GitHub
        let mut request = client
            .get(RUST_GITHUB_API)
            .header("User-Agent", "VersionWatch/1.0")
            .header("Accept", "application/vnd.github.v3+json");

        // Ajouter le token GitHub si disponible
        if let Some(token) = &self.github_token {
            request = request.bearer_auth(token);
        }

        let response = request
            .send()
            .await
            .map_err(|e| Error::Other(anyhow::anyhow!("Failed to fetch Rust releases: {e}")))?;

        let releases: Vec<GitHubRelease> = response
            .json()
            .await
            .map_err(|e| Error::Other(anyhow::anyhow!("Failed to parse Rust releases: {e}")))?;

        // Convertir en ProductCycle
        let mut cycles = Vec::new();
        for release in releases {
            // Ignorer les prereleases et ne garder que les versions stables
            if release.prerelease {
                continue;
            }

            // Nettoyer le nom de version (enlever le préfixe 'v' si présent)
            let version = release
                .tag_name
                .strip_prefix('v')
                .unwrap_or(&release.tag_name);

            // Ne garder que les versions qui ressemblent à des versions Rust (x.y.z)
            if version.matches('.').count() >= 1 {
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
