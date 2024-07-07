#!/bin/sh

# Function to wait for the database to be ready
wait_for_db() {
  local host="$1"
  local port="$2"
  local timeout="$3"
  local start_time=$(date +%s)

  while true; do
    if nc -z "$host" "$port"; then
      echo "Database connection established."
      break
    else
      local current_time=$(date +%s)
      if [ $((current_time - start_time)) -ge "$timeout" ]; then
        echo "Timed out waiting for DB: $host:$port"
        exit 1
      fi
      echo "Waiting for database connection..."
      sleep 1
    fi
  done
}

# Environment variables
DATABASE_HOST=${DATABASE_HOST:-inv-db}
DATABASE_PORT=${DATABASE_PORT:-5432}
DATABASE_TIMEOUT=${DATABASE_TIMEOUT:-30}

# Wait for the database to be ready
wait_for_db "$DATABASE_HOST" "$DATABASE_PORT" "$DATABASE_TIMEOUT"

# Run any necessary setup commands
# For example, you might want to run database migrations here
# ./your_migration_command
cargo run build --release
/usr/src/inv-server/target/release/app

# Start the Rust application
exec "$@"