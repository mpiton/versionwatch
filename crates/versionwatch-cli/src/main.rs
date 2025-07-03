use anyhow::Result;
use polars::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_subscriber::{EnvFilter, prelude::*};
use versionwatch_collect::{
    Collector, apache::ApacheCollector, eclipse_temurin::EclipseTemurinCollector,
    github::GitHubCollector, go::GoCollector, node::NodeCollector, perl::PerlCollector,
    php::PhpCollector,
};
use versionwatch_config::{Source, load as load_config};
use versionwatch_core::domain::software_version::SoftwareVersion;
use versionwatch_db::Db as Database;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    // 1. Improved Logging Setup
    // Initialize tracing with EnvFilter to allow log level control via RUST_LOG
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let config = load_config(std::path::Path::new("config/base.yml"))?;
    let github_token = std::env::var("GITHUB_TOKEN").ok();

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost:5432/versionwatch".to_string());
    let db = Arc::new(Mutex::new(Database::connect(&db_url).await?));

    let mut collectors: Vec<Arc<dyn Collector>> = vec![];
    for target in &config.targets {
        if !target.enabled {
            continue;
        }

        let collector: Option<Arc<dyn Collector>> = if let Some(source) = &target.source {
            match source {
                Source::Github {
                    repository,
                    github_source,
                    cleaning,
                } => {
                    let collector = GitHubCollector::new(
                        target.name.clone(),
                        repository.clone(),
                        github_token.clone(),
                        github_source.clone(),
                        cleaning.clone(),
                    );
                    Some(Arc::new(collector))
                }
            }
        } else {
            // Fallback to old mechanism for non-migrated collectors
            let collector: Arc<dyn Collector> = match target.name.as_str() {
                "node" => Arc::new(NodeCollector {}),
                "eclipse-temurin" => Arc::new(EclipseTemurinCollector {}),
                "go" => Arc::new(GoCollector {}),
                "php" => Arc::new(PhpCollector::default()),
                "perl" => Arc::new(PerlCollector {}),
                "apache" => Arc::new(ApacheCollector::new()),
                _ => {
                    tracing::warn!("Unknown target without a 'type' field: '{}'", target.name);
                    continue;
                }
            };
            Some(collector)
        };

        if let Some(collector) = collector {
            collectors.push(collector);
        }
    }

    let mut handles = vec![];
    for collector in collectors {
        let handle = tokio::spawn(async move {
            let collector_name = collector.name();
            tracing::info!(collector = collector_name, "Collecting versions...");
            (collector_name.to_string(), collector.collect().await)
        });
        handles.push(handle);
    }

    let mut all_dataframes = vec![];
    let mut failed_targets = vec![];

    for handle in handles {
        let (collector_name, result) = handle.await.unwrap();
        match result {
            Ok(df) => all_dataframes.push(df),
            Err(e) => {
                failed_targets.push(collector_name.clone());
                tracing::error!(
                    collector = collector_name,
                    "Failed to collect versions: {:?}",
                    e
                );
            }
        }
    }

    if all_dataframes.is_empty() {
        tracing::info!("No data collected. Exiting.");
        return Ok(());
    }

    let mut final_df = all_dataframes.remove(0);
    for df in all_dataframes {
        final_df.vstack_mut(&df)?;
    }

    // --- Phase 2: Data Transformation ---
    tracing::info!("Cleaning collected data...");
    let cleaned_df = final_df
        .lazy()
        .with_column(
            // Clean up the LTS version string for Eclipse Temurin
            when(col("name").eq(lit("eclipse-temurin")))
                .then(col("latest_lts_version").map(
                    |s| {
                        let transformed = s.str().unwrap().apply(|opt_v| {
                            opt_v.map(|v| {
                                v.trim_end_matches("-LTS")
                                    .trim_end_matches(".LTS")
                                    .to_string()
                                    .into()
                            })
                        });
                        Ok(Some(transformed.into_series().into()))
                    },
                    GetOutput::from_type(DataType::String),
                ))
                .otherwise(col("latest_lts_version"))
                .alias("latest_lts_version"),
        )
        .collect()?;

    println!("{cleaned_df}");

    tracing::info!("Loading data into the database...");
    let db_locked = db.lock().await;

    // Extract columns from the DataFrame
    let names = cleaned_df.column("name")?.str()?;
    let latest_versions = cleaned_df.column("latest_version")?.str()?;
    let latest_lts_versions = cleaned_df.column("latest_lts_version")?.str()?;
    let is_ltss = cleaned_df.column("is_lts")?.bool()?;
    let release_notes_urls = cleaned_df.column("release_notes_url")?.str()?;

    for i in 0..cleaned_df.height() {
        let version = SoftwareVersion {
            name: names.get(i).unwrap_or("").to_string(),
            latest_version: latest_versions.get(i).unwrap_or("").to_string(),
            latest_lts_version: latest_lts_versions.get(i).map(|s| s.to_string()),
            is_lts: is_ltss.get(i).unwrap_or(false),
            release_notes_url: release_notes_urls.get(i).map(|s| s.to_string()),
            // Fields not in DataFrame yet are set to default
            current_version: "".to_string(),
            eol_date: None,
            cve_count: 0,
        };

        if let Err(e) = db_locked.upsert_version(&version).await {
            tracing::error!("Failed to upsert version for '{}': {}", version.name, e);
            failed_targets.push(version.name);
        }
    }

    if !failed_targets.is_empty() {
        tracing::warn!(
            "The following collectors failed: {}",
            failed_targets.join(", ")
        );
    }

    Ok(())
}
