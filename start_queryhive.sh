#!/bin/sh

# Function to check if a container is running
is_container_running() {
    container_name=$1
    if [ "$(docker ps -q -f name=$container_name)" ]; then
        return 0
    else
        return 1
    fi
}

# Function to check if Elasticsearch is ready
is_elasticsearch_ready() {
    curl -s http://localhost:9200/_cluster/health | grep -q '"status":"yellow\|greeen"'
}

# Check if Elasticsearch is running
if is_container_running "hive-es"; then
    echo "Elasticsearch is already running."
else
    echo "Starting Elasticsearch..."
    docker-compose up -d elasticsearch
fi

# Wait for Elasticsearch to be ready
echo "Waiting for Elasticsearch to be ready..."
until is_elasticsearch_ready; do
    echo "Elasticsearch is not ready yet. Waiting..."
    sleep 10
done
echo "Elasticsearch is ready."

# Check if Kibana is running
if is_container_running "hive-kibana"; then
    echo "Kibana is already running."
else
    echo "Starting Kibana..."
    docker-compose up -d kibana
fi

# Check if Eland is running
if is_container_running "hive-eland"; then
    echo "Eland is already running."
else
    echo "Starting Eland..."
    docker-compose up -d eland
    # Wait for Eland to be ready
    echo "Waiting for Eland to be ready..."
    sleep 10  # Adjust the sleep duration as needed
fi

# Run the Eland script
echo "Running the Eland script..."
docker-compose run --rm eland eland_import_hub_model --url http://hive-es:9200/ --hub-model-id sentence-transformers/all-MiniLM-L6-v2 --task-type ner


# Start the backend
echo "Starting the backend..."
pwd 
cd ./backend
export APP_ENV=local
export RUST_LOG=info
cargo run & echo $! > ../backend.pid

# Start the frontend
echo "Starting the frontend..."
pwd
cd ../frontend
pwd
elm reactor --port=3002 &  echo $! > ../frontend.pid

open http://localhost:3002/src/Main.elm 