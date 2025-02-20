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


# Parse command-line arguments
NO_DOCKER=false
while [ "$1" != "" ]; do
    case $1 in
        --no-docker ) NO_DOCKER=true
                      ;;
        * )           echo "Invalid option: $1"
                      exit 1
    esac
    shift
done

# Stop the backend
echo "Stopping the backend..."
stop_process_by_pid_file "./backend.pid"

# Stop the frontend
echo "Stopping the frontend..."
stop_process_by_pid_file "./frontend.pid"


# Stop Docker containers if the --no-docker flag is not set
if [ "$NO_DOCKER" = false ]; then
    echo "Stopping Elasticsearch, Kibana and Eland..."
    docker-compose stop
else
    echo "Skipping Docker container stop."
fi