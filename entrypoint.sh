#!/bin/sh

$APP_DIR/connect_db.sh
$APP_DIR/populate.sh
$APP_DIR/server/target/release/app

exec "$@"