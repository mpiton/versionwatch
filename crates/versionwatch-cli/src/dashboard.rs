use anyhow::Result;
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use versionwatch_collect::{
    apache::ApacheCollector, caddy::CaddyCollector, docker::DockerCollector,
    eclipse_temurin::EclipseTemurinCollector, go::GoCollector, kong::KongCollector,
    kotlin::KotlinCollector, mongodb::MongoDbCollector, mysql::MySqlCollector,
    nginx::NginxCollector, node::NodeCollector, perl::PerlCollector, php::PhpCollector,
    postgresql::PostgresqlCollector, python::PythonCollector, ruby::RubyCollector,
    rust::RustCollector, scala::ScalaCollector, swift::SwiftCollector, Collector,
};
use versionwatch_config::Settings;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Settings>,
    pub metrics: Arc<tokio::sync::RwLock<DashboardMetrics>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DashboardMetrics {
    pub total_collectors: usize,
    pub active_collectors: usize,
    pub failed_collectors: usize,
    pub total_versions: usize,
    pub last_updated: String,
    pub collector_stats: Vec<CollectorMetric>,
    pub system_health: SystemHealth,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CollectorMetric {
    pub name: String,
    pub version_count: usize,
    pub status: String,
    pub last_collection: String,
    pub performance_category: String,
    pub response_time: f64,
    pub error_message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SystemHealth {
    pub overall_status: String,
    pub anomalies_detected: usize,
    pub success_rate: f64,
    pub average_response_time: f64,
}

impl Default for SystemHealth {
    fn default() -> Self {
        Self {
            overall_status: "Unknown".to_string(),
            anomalies_detected: 0,
            success_rate: 0.0,
            average_response_time: 0.0,
        }
    }
}

pub async fn start_server(host: &str, port: u16, config: &Settings) -> Result<()> {
    let app_state = AppState {
        config: Arc::new(config.clone()),
        metrics: Arc::new(tokio::sync::RwLock::new(DashboardMetrics::default())),
    };

    // Start background metrics collection
    let metrics_handle = app_state.metrics.clone();
    let config_handle = app_state.config.clone();
    tokio::spawn(async move {
        collect_metrics_periodically(metrics_handle, config_handle).await;
    });

    let app = Router::new()
        // API routes (doivent être avant les fichiers statiques)
        .route("/api/metrics", get(get_metrics))
        .route("/api/health", get(health_check))
        // Servir les fichiers statiques depuis frontend/dist
        .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
        .route("/vite.svg", get(serve_vite_svg))
        .route("/logo.png", get(serve_logo_png))
        // Route de fallback pour servir index.html pour toutes les autres routes
        .fallback(serve_index)
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;
    println!("🚀 Starting VersionWatch dashboard on http://{host}:{port}");
    println!("🌐 Dashboard running on http://{host}:{port}");
    println!("📊 Metrics API available at http://{host}:{port}/api/metrics");
    println!("🔍 Health check at http://{host}:{port}/api/health");

    axum::serve(listener, app).await?;
    Ok(())
}

async fn get_metrics(State(state): State<AppState>) -> impl IntoResponse {
    let metrics = state.metrics.read().await;
    axum::Json(metrics.clone())
}

async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    let metrics = state.metrics.read().await;
    axum::Json(serde_json::json!({
        "status": "healthy",
        "active_collectors": metrics.active_collectors,
        "total_collectors": metrics.total_collectors,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn serve_index() -> impl IntoResponse {
    match tokio::fs::read_to_string("frontend/dist/index.html").await {
        Ok(content) => axum::response::Html(content),
        Err(_) => axum::response::Html(
            "<html><body><h1>Error: Could not load frontend</h1></body></html>".to_string(),
        ),
    }
}

async fn serve_vite_svg() -> impl IntoResponse {
    match tokio::fs::read("frontend/dist/vite.svg").await {
        Ok(content) => {
            let headers = [
                ("content-type", "image/svg+xml"),
                ("cache-control", "public, max-age=31536000"),
            ];
            (headers, content).into_response()
        }
        Err(_) => (axum::http::StatusCode::NOT_FOUND, "SVG not found").into_response(),
    }
}

async fn serve_logo_png() -> impl IntoResponse {
    match tokio::fs::read("frontend/dist/logo.png").await {
        Ok(content) => {
            let headers = [
                ("content-type", "image/png"),
                ("cache-control", "public, max-age=31536000"),
            ];
            (headers, content).into_response()
        }
        Err(_) => (axum::http::StatusCode::NOT_FOUND, "Logo not found").into_response(),
    }
}

// Background metrics collection
async fn collect_metrics_periodically(
    metrics: Arc<tokio::sync::RwLock<DashboardMetrics>>,
    config: Arc<Settings>,
) {
    let mut interval = interval(Duration::from_secs(300)); // Update every 5 minutes

    loop {
        interval.tick().await;

        match collect_current_metrics(&config).await {
            Ok(new_metrics) => {
                let mut metrics_guard = metrics.write().await;
                *metrics_guard = new_metrics;
                println!("📊 Metrics updated successfully");
            }
            Err(e) => {
                eprintln!("❌ Failed to update metrics: {e}");
            }
        }
    }
}

async fn collect_current_metrics(config: &Settings) -> Result<DashboardMetrics> {
    let start_time = std::time::Instant::now();
    let mut collector_stats = Vec::new();
    let mut total_versions = 0;
    let mut successful_collections = 0;
    let mut total_collections = 0;

    for target in &config.targets {
        if !target.enabled {
            continue;
        }

        total_collections += 1;
        let collector = create_collector(target, config.github_token.as_deref());

        if let Some(collector) = collector {
            let collector_start_time = std::time::Instant::now();
            match collector.collect().await {
                Ok(df) => {
                    let response_time = collector_start_time.elapsed().as_millis() as f64;
                    let version_count = df.height();
                    total_versions += version_count;
                    successful_collections += 1;

                    let performance_category = match version_count {
                        0 => "No Data",
                        1..=10 => "Low Volume",
                        11..=50 => "Medium Volume",
                        51..=200 => "High Volume",
                        _ => "Very High Volume",
                    };

                    collector_stats.push(CollectorMetric {
                        name: target.name.clone(),
                        version_count,
                        status: "Active".to_string(),
                        last_collection: chrono::Utc::now().to_rfc3339(),
                        performance_category: performance_category.to_string(),
                        response_time,
                        error_message: None,
                    });
                }
                Err(e) => {
                    let response_time = collector_start_time.elapsed().as_millis() as f64;
                    collector_stats.push(CollectorMetric {
                        name: target.name.clone(),
                        version_count: 0,
                        status: "Failed".to_string(),
                        last_collection: chrono::Utc::now().to_rfc3339(),
                        performance_category: "No Data".to_string(),
                        response_time,
                        error_message: Some(format!("{e}")),
                    });
                }
            }
        }
    }

    let total_collectors = config.targets.iter().filter(|t| t.enabled).count();
    let active_collectors = collector_stats
        .iter()
        .filter(|c| c.status == "Active")
        .count();
    let success_rate = if total_collections > 0 {
        (successful_collections as f64 / total_collections as f64) * 100.0
    } else {
        0.0
    };

    let average_response_time = start_time.elapsed().as_millis() as f64 / total_collections as f64;

    let version_counts: Vec<usize> = collector_stats.iter().map(|c| c.version_count).collect();
    let avg_versions = if !version_counts.is_empty() {
        version_counts.iter().sum::<usize>() as f64 / version_counts.len() as f64
    } else {
        0.0
    };

    let anomalies_detected = collector_stats
        .iter()
        .filter(|c| c.version_count > 0 && (c.version_count as f64) < avg_versions * 0.3)
        .count();

    let overall_status = if success_rate >= 90.0 {
        "Healthy"
    } else if success_rate >= 70.0 {
        "Warning"
    } else {
        "Critical"
    };

    let system_health = SystemHealth {
        overall_status: overall_status.to_string(),
        anomalies_detected,
        success_rate,
        average_response_time,
    };

    Ok(DashboardMetrics {
        total_collectors,
        active_collectors,
        failed_collectors: total_collectors - active_collectors,
        total_versions,
        last_updated: chrono::Utc::now().to_rfc3339(),
        collector_stats,
        system_health,
    })
}

fn create_collector(
    target: &versionwatch_config::Target,
    github_token: Option<&str>,
) -> Option<Box<dyn Collector + Send + Sync>> {
    match target.name.as_str() {
        "apache" => Some(Box::new(ApacheCollector::new())),
        "docker" => Some(Box::new(DockerCollector::new(&target.name))),
        "eclipse-temurin" => Some(Box::new(EclipseTemurinCollector::new(&target.name))),
        "go" => Some(Box::new(GoCollector::new(&target.name))),
        "mongodb" => Some(Box::new(MongoDbCollector::new(&target.name))),
        "mysql" => {
            if let Some(token) = github_token {
                Some(Box::new(MySqlCollector::with_token(
                    &target.name,
                    token.to_string(),
                )))
            } else {
                Some(Box::new(MySqlCollector::new(&target.name)))
            }
        }
        "node" => Some(Box::new(NodeCollector::new(&target.name))),
        "perl" => Some(Box::new(PerlCollector)),
        "php" => Some(Box::new(PhpCollector::new(&target.name))),
        "postgresql" => Some(Box::new(PostgresqlCollector::new(&target.name))),
        "swift" => Some(Box::new(SwiftCollector::new(&target.name))),
        "kong" => Some(Box::new(KongCollector::new(&target.name))),
        "caddy" => Some(Box::new(CaddyCollector::new(&target.name))),
        "kotlin" => Some(Box::new(KotlinCollector::new(&target.name))),
        "nginx" => Some(Box::new(NginxCollector::new(&target.name))),
        "python" => Some(Box::new(PythonCollector::new(&target.name))),
        "ruby" => Some(Box::new(RubyCollector::new(&target.name))),
        "rust" => Some(Box::new(RustCollector::new(&target.name))),
        "scala" => Some(Box::new(ScalaCollector::new(&target.name))),
        _ => None,
    }
}
