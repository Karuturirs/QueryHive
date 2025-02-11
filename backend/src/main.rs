use axum::{routing::{get}, Router};
use hyper::Server;
use log::{info, error};
use std::net::SocketAddr;
use crate::health::{liveness, readiness, health_check};
use crate::metrics::metrics;
use crate::setup::{setup};

mod health;
mod metrics;
mod setup;
mod config;

#[tokio::main]
pub async fn main() {
    // Setup logging and configuration
    setup().await;

    // Set up Axum routes
    let app = Router::new()
        .route("/liveness", get(liveness))
        .route("/readiness", get(readiness))
        .route("/health", get(health_check))
        .route("/metrics", get(metrics));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("🚀 The server is ready to accept requests on {}", addr);

    // Start the server
    if let Err(e) = Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        error!("Server error: {}", e);
    }
}
