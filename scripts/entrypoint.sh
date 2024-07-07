#!/bin/sh

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

DATABASE_HOST=${DATABASE_HOST:-inv-db}
DATABASE_PORT=${DATABASE_PORT:-5432}
DATABASE_TIMEOUT=${DATABASE_TIMEOUT:-30}
POSTGRES_USER=${POSTGRES_USER:-some-postgres}
POSTGRES_DB=${POSTGRES_DB:-some-postgres}
POSTGRES_PASSWORD=${POSTGRES_DB:-mysecretpassword}

export PGPASSWORD=$POSTGRES_PASSWORD

wait_for_db "$DATABASE_HOST" "$DATABASE_PORT" "$DATABASE_TIMEOUT"
rm -rf container documentation LICENSE README.md docker-compose.yml .github .vscode

# remove if data is already done
cd /app/scripts/excel_to_sql
cargo run build --release

/app/scripts/excel_to_sql/target/release/excel_to_sql

cd /app/server
cargo run build --release

/app/server/target/release/app

exec "$@"