# QueryHive 🧠✨
A RAG application built for quering day to day needs

#### Features:
 * Support of openAI & Opensource LLM
 * Chuncking/Embedding - cost effective
 * Enabling upload for large documents and data
 * CLI options
 * Scalability 


 > Audience: Tech/Business (with basic sofware experience)

### Design:

![Flow Diagram](./samples/design.png)

#### Tech Stack:

* UI - ELM
* Backend Services - Rust/Python
* VectorDB - Elastic
* LLM - Opensource/OpenAPI



#### Setup:


pip install sentence-transformers transformers openai tiktoken nltk


cargo add pyo3 --features "extension-module"
cargo add pyo3 --features "auto-initialize"


export RUST_LOG=info  # This will enable logs at info level and higher
cargo run
