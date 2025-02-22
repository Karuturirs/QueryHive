use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    response::Json as AxumJson,
    response::sse::{Event, Sse},
    routing::{get, get_service, post},
    Router,
};
use hyper::Server;
use log::{info, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashSet, net::SocketAddr};
use tower_http::services::ServeDir;
use tower_http::cors::{Any, CorsLayer};
use crate::health::{liveness, readiness, health_check};
use crate::config::Config;
use crate::metrics::metrics;
use chrono::Utc;
use futures::stream::{Stream, StreamExt};
use std::convert::Infallible;
use std::time::Duration;
use tokio::time::interval;

#[derive(Debug, Serialize, Deserialize)]
struct SearchParams {
    query: String,
}


#[derive(Deserialize)]
struct ChatInput {
    chatInput: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Document {
    id: Option<String>,
    title: String,
    content: String,
    path: String,
    tags: Vec<String>,
    created_at: String,
}

#[derive(Deserialize)]
struct ElasticsearchResponse {
    hits: HitsWrapper,
}

#[derive(Deserialize)]
struct HitsWrapper {
    hits: Vec<Hit>,
}

#[derive(Deserialize)]
struct Hit {
    _id: String,
    _source: Document,
}

pub async fn run(config: Config) {
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

    let app = Router::new()
        .route("/search", get(query))
        .route("/chat", post(chat_handler))
        .route("/documents", post(add_document).get(get_documents))
        .layer(cors)
        .with_state(config);

    // Set up Axum routes
    let app_router = Router::new()
        .nest_service("/", frontend_service)
        .nest("/api", app)
        .route("/api/liveness", get(liveness))
        .route("/api/readiness", get(readiness))
        .route("/api/health", get(health_check))
        .route("/api/metrics", get(metrics));

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

async fn query(State(state): State<Config>, Query(params): Query<SearchParams>) -> Result<AxumJson<Value>, (StatusCode, AxumJson<Value>)> {
    let results = &params.query;
    Ok(AxumJson(json!({ "results": results })))
}

async fn chat_handler(Json(payload): Json<ChatInput>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut interval = interval(Duration::from_secs(1));
    let chat_input = payload.chatInput.clone();

    let stream = async_stream::stream! {
        yield Ok(Event::default().data(format!("AI: Processing '{}'", chat_input)));
        interval.tick().await;
        yield Ok(Event::default().data("AI: Response part 1"));
        interval.tick().await;
        yield Ok(Event::default().data("AI: Response part 2"));
        interval.tick().await;
        yield Ok(Event::default().data("AI: Response part 3"));
    };

    Sse::new(stream)
}

async fn add_document(Json(mut doc): Json<Document>) -> AxumJson<String> {
    let client = Client::new();
    doc.tags = generate_tags(&doc.title, &doc.content, &doc.path);
    doc.created_at = Utc::now().to_rfc3339();

    let res = client
        .post("http://localhost:9200/documents/_doc")
        .json(&doc)
        .send()
        .await;

    match res {
        Ok(response) => AxumJson(format!("Document added: {:?}", response.status())),
        Err(_) => AxumJson("Failed to add document".to_string()),
    }
}

async fn get_documents() -> AxumJson<Vec<Document>> {
    let client = Client::new();
    let res = client
        .get("http://localhost:9200/documents/_search?size=1000")
        .send()
        .await
        .unwrap()
        .json::<ElasticsearchResponse>()
        .await
        .unwrap();

    let documents: Vec<Document> = res
        .hits
        .hits
        .into_iter()
        .map(|hit| Document {
            id: Some(hit._id),
            ..hit._source
        })
        .collect();

    AxumJson(documents)
}

fn generate_tags(title: &str, content: &str, path: &str) -> Vec<String> {
    let mut tags: HashSet<String> = HashSet::new();

    // Extract words from title and content
    for word in title.split_whitespace().chain(content.split_whitespace()) {
        if word.len() > 3 {
            tags.insert(word.to_lowercase());
        }
    }

    // Extract path-based tags
    for folder in path.split('/') {
        if !folder.is_empty() {
            tags.insert(folder.to_lowercase());
        }
    }

    let mut tags_vec: Vec<String> = tags.into_iter().collect();
    tags_vec.sort();
    tags_vec
}