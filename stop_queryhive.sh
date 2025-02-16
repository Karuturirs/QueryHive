#!/bin/sh

# Function to stop a process by PID file
stop_process_by_pid_file() {
    pid_file=$1
    if [ -f $pid_file ]; then
        pid=$(cat $pid_file)
        echo "Stopping process with PID: $pid"
        kill $pid
        rm $pid_file
    else
        echo "PID file $pid_file not found."
    fi
}

# Stop the backend
echo "Stopping the backend..."
stop_process_by_pid_file "./backend.pid"

# Stop the frontend
echo "Stopping the frontend..."
stop_process_by_pid_file "./frontend.pid"

# Stop Docker containers
echo "Stopping Elasticsearch, Kibana and Eland..."
docker-compose stop