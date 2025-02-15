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

# Check if Elasticsearch is running
if is_container_running "hive-es"; then
    echo "Elasticsearch is already running."
else
    echo "Starting Elasticsearch..."
    docker-compose up -d elasticsearch
    # Wait for Elasticsearch and Kibana to be ready
    echo "Waiting for Elasticsearch and Kibana to be ready..."
    sleep 10  # Adjust the sleep duration as needed
fi

# Check if Kibana is running
if is_container_running "hive-kibana"; then
    echo "Kibana is already running."
else
    echo "Starting Kibana..."
    docker-compose up -d kibana
fi



# Start the backend
echo "Starting the backend..."
pwd 
cd ./backend
pwd
nohup cargo run & echo $! > ../backend.pid

# Start the frontend
echo "Starting the frontend..."
pwd
cd ../frontend
pwd
elm reactor --port=3002 &  echo $! > ../frontend.pid