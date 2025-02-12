
use axum::{
    extract:: {Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, get_service},
    Router,
};
use hyper::Server;
use log::{info, error};
use std::net::SocketAddr;
use crate::health::{liveness, readiness, health_check};
use crate::config::Config;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::services::ServeDir;
use tower_http::cors::{Any, CorsLayer};


#[derive(Debug, Serialize, Deserialize)]
struct SearchParams {
    query: String,
}


pub async fn run(config:Config) {

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

     // Serve static files from the `static` folder
     let frontend_service = get_service(ServeDir::new("./static"))
     .handle_error(|error| async move {
         (
             axum::http::StatusCode::INTERNAL_SERVER_ERROR,
             format!("Unhandled internal error: {}", error),
         )
     });

    let app = Router::new().route("/search", get(query)).layer(cors).with_state(config);

    // Set up Axum routes
    let app_router = Router::new()
        .nest_service("/", frontend_service)
        .nest("/api", app)
        .route("/api/liveness", get(liveness))
        .route("/api/readiness", get(readiness))
        .route("/api/health", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("🚀 The server is ready to accept requests on {}", addr);

    // Start the server
    if let Err(e) = Server::bind(&addr)
        .serve(app_router.into_make_service())
        .await
    {
        error!("Server error: {}", e);
    }
}

async fn query( State(state): State<Config>, Query(params): Query<SearchParams>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let results = &params.query;
    
    Ok(Json(json!({ "results": results })))
         
}