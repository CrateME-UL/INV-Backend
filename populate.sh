#!/bin/sh

echo "APP_DIR: $APP_DIR"
echo "DATABASE_URL: $DATABASE_URL"
echo "Script Path: $APP_DIR/scripts/db_script.sql"
echo "Executable Path: $APP_DIR/scripts/excel_to_sql/target/debug/excel_to_sql"

if [ -f "$APP_DIR/scripts/db_script.sql" ]; then
    echo "SQL script file found: $APP_DIR/scripts/db_script.sql"
else
    echo "Error: SQL script file not found: $APP_DIR/scripts/db_script.sql"
    exit 1
fi

if [ -f "$APP_DIR/scripts/excel_to_sql/target/debug/excel_to_sql" ]; then
    echo "Executable file found: $APP_DIR/scripts/excel_to_sql/target/debug/excel_to_sql"
else
    echo "Error: Executable file not found: $APP_DIR/scripts/excel_to_sql/target/debug/excel_to_sql"
    exit 1
fi

sql_script=$(cat "$APP_DIR/scripts/db_script.sql")
psql "$DATABASE_URL" -c "$sql_script"

echo "Running executable..."
$APP_DIR/scripts/excel_to_sql/target/debug/excel_to_sql

exec "$@"
