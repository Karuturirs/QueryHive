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

const DATA_FOLDER: &str = "../data";
const INDEX_NAME: &str = "qh-index";
const PIPELINE_NAME: &str = "qh-pipeline";

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

async fn create_pipeline(client: &Client) -> Result<(), reqwest::Error> {
    let response = client.get(&format!("http://localhost:9200/_ingest/pipeline/{}", PIPELINE_NAME))
        .send()
        .await?;

    if response.status().is_success() {
        info!("Pipeline already exists.");
        return Ok(());
    }

    let pipeline = json!({
        "description": "Pipeline to chunk documents and create vector embeddings",
        "processors": [
            {
                "script": {
                    "description": "Chunk body_content into sentences by looking for . followed by a space",
                    "lang": "painless",
                    "source": r#"
                        String[] envSplit = /((?<!M(r|s|rs)\.) (?<=\.) |(?<=\!) |(?<=\?) )/.split(ctx['content']);
                        ctx['passages'] = new ArrayList();
                        int i = 0;
                        boolean remaining = true;
                        if (envSplit.length == 0) {
                            return;
                        } else if (envSplit.length == 1) {
                            Map passage = ['text': envSplit[0]];
                            ctx['passages'].add(passage);
                        } else {
                            while (remaining) {
                                Map passage = ['text': envSplit[i++]];
                                while (i < envSplit.length && passage.text.length() + envSplit[i].length() < params.model_limit) {
                                    passage.text = passage.text + ' ' + envSplit[i++];
                                }
                                if (i == envSplit.length) {
                                    remaining = false;
                                }
                                ctx['passages'].add(passage);
                            }
                        }
                    "#,
                    "params": {
                        "model_limit": 400
                    }
                }
            },
            {
                "foreach": {
                    "field": "passages",
                    "processor": {
                        "inference": {
                            "field_map": {
                                "_ingest._value.text": "text_field"
                            },
                            "model_id": "sentence-transformers__all-minilm-l6-v2",
                            "target_field": "_ingest._value.vector",
                            "on_failure": [
                                {
                                    "append": {
                                        "field": "_source._ingest.inference_errors",
                                        "value": [
                                            {
                                                "message": "Processor 'inference' in pipeline 'qh-pipeline' failed with message '{{ _ingest.on_failure_message }}'",
                                                "pipeline": "qh-pipeline",
                                                "timestamp": "{{{ _ingest.timestamp }}}"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                }
            }
        ]
    });

    client.put(&format!("http://localhost:9200/_ingest/pipeline/{}", PIPELINE_NAME))
        .json(&pipeline)
        .send()
        .await?;

    info!("Pipeline created successfully.");
    Ok(())
}

async fn create_index(client: &Client) -> Result<(), reqwest::Error> {
    let response = client.head(&format!("http://localhost:9200/{}", INDEX_NAME))
        .send()
        .await?;

    if response.status().is_success() {
        info!("Index already exists.");
        return Ok(());
    }

    let index = json!({
        "settings": {
            "number_of_shards": 1,
            "number_of_replicas": 0
        },
        "mappings": {
            "properties": {
                "content": { "type": "text" },
                "metadata": {
                    "properties": {
                        "path": { "type": "keyword" },
                        "created_date": { "type": "date" },
                        "last_modified": { "type": "date" },
                        "title": { "type": "text" },
                        "description": { "type": "text" }
                    }
                },
                "passages": {
                    "type": "nested",
                    "properties": {
                        "text": { "type": "text" },
                        "vector": {
                            "type": "dense_vector",
                            "dims": 384,
                            "similarity": "dot_product"
                        }
                    }
                }
            }
        }
    });

    client.put(&format!("http://localhost:9200/{}", INDEX_NAME))
        .json(&index)
        .send()
        .await?;

    info!("Index created successfully.");
    Ok(())
}

async fn index_document(client: &Client, document: &Document) -> Result<(), reqwest::Error> {
    client.post(&format!("http://localhost:9200/{}/_doc?pipeline={}", INDEX_NAME, PIPELINE_NAME))
        .json(document)
        .send()
        .await?;

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

async fn index_files(client: &Client, path: &Path) -> io::Result<()> {
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

            index_document(client, &document).await.unwrap();
        }
    }

    info!("Indexed files: {:?}", indexed_files);
    Ok(())
}

async fn check_for_updates(client: &Client, path: &Path) -> io::Result<()> {
    let mut files_metadata: HashMap<String, FileMetadata> = HashMap::new();
    let mut updated_files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let last_modified = metadata.modified()?;

            if let Some(file_metadata) = files_metadata.get(&path.to_string_lossy().to_string()) {
                if file_metadata.last_modified < last_modified {
                    let content = read_file_content(&path)?;

                    let document = Document {
                        id: path.file_name().unwrap().to_string_lossy().to_string(),
                        content,
                        metadata: file_metadata.clone(),
                    };

                    index_document(client, &document).await.unwrap();
                    updated_files.push(file_metadata.path.clone());
                }
            } else {
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
                updated_files.push(file_metadata.path.clone());

                index_document(client, &document).await.unwrap();
            }
        }
    }

    info!("Updated files: {:?}", updated_files);
    Ok(())
}

pub async fn start_indexing() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Create the pipeline if it doesn't exist
    create_pipeline(&client).await?;

    // Create the index if it doesn't exist
    create_index(&client).await?;

    // Index the initial files
    let start_time = Instant::now();
    let start_datetime = Utc::now();
    info!("Indexing started at {}", start_datetime);

    index_files(&client, Path::new(DATA_FOLDER)).await?;

    let end_time = Instant::now();
    let end_datetime = Utc::now();
    let duration = end_time.duration_since(start_time);
    info!("Indexing completed at {}", end_datetime);
    info!("Indexing duration: {:?}", duration);

    // Schedule the cron job to check for updates every hour
    let sched = JobScheduler::new().await?;
    let job = Job::new_async("0 * * * * *", move |_uuid, _l| {
        let client = client.clone();
        Box::pin(async move {
            let start_time = Instant::now();
            let start_datetime = Utc::now();
            info!("Update check started at {}", start_datetime);

            check_for_updates(&client, Path::new(DATA_FOLDER)).await.unwrap();

            let end_time = Instant::now();
            let end_datetime = Utc::now();
            let duration = end_time.duration_since(start_time);
            info!("Update check completed at {}", end_datetime);
            info!("Update check duration: {:?}", duration);
        })
    })?;
    sched.add(job).await?;
    sched.start().await?;

    Ok(())
}
