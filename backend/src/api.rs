use axum::{
    routing::{get},
    Router,
};
use hyper::Server;
use log::{info, error};
use prometheus::{Encoder, TextEncoder, register_int_counter, IntCounter};
use std::net::SocketAddr;
use tokio::sync::OnceCell;

static HEALTH_CHECK_COUNTER: OnceCell<IntCounter> = OnceCell::const_new();

async fn liveness() -> &'static str {
    info!("Liveness check hit");
    "OK"
}

async fn readiness() -> &'static str {
    info!("Readiness check hit");
    "OK"
}

async fn health_check() -> &'static str {
    info!("Health check hit");
    "OK"
}

// Prometheus metrics endpoint
async fn metrics() -> String {
    let counter = HEALTH_CHECK_COUNTER.get().unwrap();
    counter.inc();

    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

async fn setup() {
    env_logger::init();
    info!("Setting up the app");

    // Register Prometheus counter for health checks
   let _ = HEALTH_CHECK_COUNTER.set(register_int_counter!("health_check_requests", "Total health check requests").unwrap());
}


pub async fn run() {
    setup().await;

    let app = Router::new()
        .route("/liveness", get(liveness))
        .route("/readiness", get(readiness))
        .route("/health", get(health_check))
        .route("/metrics", get(metrics));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("Server running at {}", addr);
    if let Err(e) = Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        error!("Server error: {}", e);
    }
}
