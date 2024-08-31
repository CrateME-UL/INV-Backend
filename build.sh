#!/bin/sh

cd $APP_DIR/scripts/excel_to_sql
cargo build

cd $APP_DIR/server
cargo build --release

exec "$@"