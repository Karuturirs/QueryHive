import nltk
import openai
import tiktoken
import torch
from transformers import AutoTokenizer, AutoModel
from sentence_transformers import SentenceTransformer

# Download sentence tokenizer
nltk.download('punkt')

### ---------- CHUNKING STRATEGIES ---------- ###
def fixed_size_chunking(text, chunk_size=512, overlap=256):
    """Split text into fixed-size chunks with overlap"""
    chunks = []
    start = 0
    while start < len(text):
        end = min(start + chunk_size, len(text))
        chunks.append(text[start:end])
        start += chunk_size - overlap
    return chunks

def sentence_based_chunking(text, max_sentences=3):
    """Split text into chunks based on sentences"""
    sentences = nltk.sent_tokenize(text)
    chunks = [" ".join(sentences[i:i+max_sentences]) for i in range(0, len(sentences), max_sentences)]
    return chunks

def paragraph_based_chunking(text):
    """Split text into paragraphs"""
    return [p.strip() for p in text.split("\n\n") if p.strip()]

def token_based_chunking(text, max_tokens=512):
    """Split text into chunks based on tokens using OpenAI's tiktoken"""
    tokenizer = tiktoken.get_encoding("cl100k_base")
    tokens = tokenizer.encode(text)
    chunks = [tokens[i:i + max_tokens] for i in range(0, len(tokens), max_tokens)]
    return [tokenizer.decode(chunk) for chunk in chunks]

### ---------- EMBEDDING OPTIONS ---------- ###
def get_embedding_openai(text):
    """Generate embedding using OpenAI API (Requires API Key)"""
    response = openai.embeddings.create(
        input=text,
        model="text-embedding-3-small"
    )
    return response.data[0].embedding

def get_embedding_sentence_transformers(text, model_name="all-MiniLM-L6-v2"):
    """Generate embedding using SentenceTransformers"""
    model = SentenceTransformer(model_name)
    return model.encode(text).tolist()

def get_embedding_huggingface(text, model_name="BAAI/bge-base-en-v1.5"):
    """Generate embedding using Hugging Face Transformer Model"""
    tokenizer = AutoTokenizer.from_pretrained(model_name)
    model = AutoModel.from_pretrained(model_name)
    
    tokens = tokenizer(text, return_tensors="pt", padding=True, truncation=True)
    with torch.no_grad():
        output = model(**tokens)
    embedding = output.last_hidden_state[:, 0, :].numpy().tolist()  # Extract CLS token
    return embedding

### ---------- MAIN FUNCTION ---------- ###
if __name__ == "__main__":
    sample_text = """Artificial Intelligence (AI) is transforming industries worldwide. From healthcare to finance, AI applications are revolutionizing how we interact with technology. 
    Large Language Models (LLMs) like GPT-4 have shown significant progress in natural language understanding and generation."""

    print("\n--- Chunking Methods ---")
    print("Fixed-size:", fixed_size_chunking(sample_text))
    print("Sentence-based:", sentence_based_chunking(sample_text))
    print("Paragraph-based:", paragraph_based_chunking(sample_text))
    print("Token-based:", token_based_chunking(sample_text))

    print("\n--- Embedding Methods ---")
    print("Sentence Transformers:", get_embedding_sentence_transformers(sample_text[:512]))
    print("Hugging Face:", get_embedding_huggingface(sample_text[:512]))
    # Uncomment below if using OpenAI API Key
    # print("OpenAI Embedding:", get_embedding_openai(sample_text[:512]))
