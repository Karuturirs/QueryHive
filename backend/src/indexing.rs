use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;
use std::time::{SystemTime, Instant};
use std::collections::HashMap;
use serde_json::json;
use reqwest::Client;
use tokio_cron_scheduler::{Job, JobScheduler};
use chrono::{Utc, DateTime};
use walkdir::WalkDir;
use pdfium_render::prelude::*;
use docx_rs::*;
use pulldown_cmark::{Parser, Options, Event};
use serde::{Deserialize, Serialize};
use log::{error, info};
use anyhow::Result;
use ollama_rs::{Ollama, generation::embeddings::request::GenerateEmbeddingsRequest};
use langchain_rust::qdrant::QdrantClient;

const DATA_FOLDER: &str = "../data";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileMetadata {
    path: String,
    created_date: DateTime<Utc>,
    last_modified: SystemTime,
    title: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    id: String,
    content: String,
    metadata: FileMetadata,
}

async fn pull_models(ollama: &Ollama) -> Result<()> {
    let models = vec![
        "sentence-transformers/all-MiniLM-L6-v2",
        "vidore/colpali-v1.3",
        "Qwen/Qwen2.5-VL-7B-Instruct"
    ];

    let local_models = ollama.list_local_models().await?;

    for model in models {
        if !local_models.iter().any(|m| m.name == model) {
            ollama.pull_model(model.to_string(), false).await?;
        }
    }

    Ok(())
}

async fn index_files(client: &Client, ollama: &Ollama, qdrant: &QdrantClient, path: &Path) -> io::Result<()> {
    let mut files_metadata: HashMap<String, FileMetadata> = HashMap::new();
    let mut indexed_files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let last_modified = metadata.modified()?;
            let created_date = metadata.created().unwrap_or_else(|_| SystemTime::now());
            let created_date: DateTime<Utc> = created_date.into();

            let file_metadata = FileMetadata {
                path: path.to_string_lossy().to_string(),
                created_date,
                last_modified,
                title: path.file_name().unwrap().to_string_lossy().to_string(),
                description: String::new(), // Add logic to extract description if available
            };

            let content = read_file_content(&path)?;

            let document = Document {
                id: path.file_name().unwrap().to_string_lossy().to_string(),
                content,
                metadata: file_metadata.clone(),
            };

            files_metadata.insert(file_metadata.path.clone(), file_metadata.clone());
            indexed_files.push(file_metadata.path.clone());

            // Embed the content using Ollama
            let embedding_request = GenerateEmbeddingsRequest::new("sentence-transformers/all-MiniLM-L6-v2".to_string(), document.content.clone());
            let embedding = ollama.generate_embeddings(embedding_request).await.unwrap();

            // Store the embedding in Qdrant
            qdrant.store_embedding(&document.id, &embedding).await.unwrap();

            index_document(client, &document).await.unwrap();
        }
    }

    info!("Indexed files: {:?}", indexed_files);
    Ok(())
}

fn read_file_content(path: &Path) -> io::Result<String> {
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
    let mut content = String::new();

    match extension {
        "md" => {
            let mut file = File::open(path)?;
            file.read_to_string(&mut content)?;
            let parser = Parser::new_ext(&content, Options::all());
            content = parser.map(|event| match event {
                Event::Text(text) => text.to_string(),
                _ => String::new(),
            }).collect::<Vec<_>>().join("");
        }
    /*  "pdf" => {
            let pdfium = Pdfium::bind_to_system_library()?;
            let document = pdfium.load_pdf_from_file(path)?;
            for page_index in 0..document.page_count() {
                let page = document.page_at_index(page_index)?;
                let textpage = page.textpage()?;
                let page_text = textpage.all_text_unencoded()?;
                content.push_str(&page_text);
                content.push_str("\n\n"); 
            }
        }
        "docx" => {
            let file = File::open(path)?;
            let mut reader = io::BufReader::new(file);
            let docx = Docx::read(&mut reader).unwrap();
            let mut docx_content = String::new();
            docx.read_to_string(&mut docx_content).unwrap();
            content.push_str(&docx_content);
        } */
        _ => {
            let mut file = File::open(path)?;
            file.read_to_string(&mut content)?;
        }
    }

    Ok(content)
}

async fn index_document(client: &Client, document: &Document) -> Result<(), reqwest::Error> {
    client.post("http://localhost:3001/documents")
        .json(document)
        .send()
        .await?;

    Ok(())
}

pub async fn start_indexing() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let ollama = Ollama::default();
    let qdrant = QdrantClient::new("http://localhost:6333");

    // Pull the models
    pull_models(&ollama).await?;

    // Index the initial files
    let start_time = Instant::now();
    let start_datetime = Utc::now();
    info!("Indexing started at {}", start_datetime);

    index_files(&client, &ollama, &qdrant, Path::new(DATA_FOLDER)).await?;

    let end_time = Instant::now();
    let end_datetime = Utc::now();
    let duration = end_time.duration_since(start_time);
    info!("Indexing completed at {}", end_datetime);
    info!("Indexing duration: {:?}", duration);

    Ok(())
}
