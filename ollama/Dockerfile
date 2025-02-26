# Use Ollama's official base image
FROM ollama/ollama:latest

# Install system dependencies
RUN apt-get update && apt-get install -y \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Install colpali-v1.2 from Hugging Face
RUN pip install --no-cache-dir huggingface_hub && \
    pip install colpali-engine==0.3.0

# Verify installation
RUN python3 -c "import colpali; print('ColPali installed:', colpali.__version__)"

CMD ["ollama", "run"]


