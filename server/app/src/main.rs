use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use dotenv::dotenv;
use serde::Serialize;
use serde_json::{Map, Number, Value};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use tower_http::cors::CorsLayer;
use tracing::instrument;
use uuid::Uuid; // Import to_bytes function

#[tokio::main(flavor = "current_thread")]
#[instrument]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/items", get(get_items))
        .layer(CorsLayer::permissive())
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_items_db(pool: &PgPool) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let items =
        sqlx::query_as("SELECT itemName, SUM(nbOfItems) as nbOfItems FROM Items GROUP BY itemName")
            .fetch_all(pool)
            .await?;
    Ok(items
        .into_iter()
        .map(|item: (String, i64)| Item {
            item_id: uuid::Uuid::new_v4(),
            item_name: item.0,
            nb_of_items: item.1 as i32,
        })
        .collect())
}

async fn get_items(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    match get_items_db(&pool).await {
        Ok(items_result) => {
            let list_response: Vec<Value> = items_result
                .into_iter()
                .map(|item| {
                    let mut map = Map::new();
                    map.insert(
                        "item_id".to_string(),
                        Value::String(item.item_id.to_string()),
                    );
                    map.insert("item_name".to_string(), Value::String(item.item_name));
                    map.insert(
                        "nb_of_items".to_string(),
                        Value::Number(Number::from(item.nb_of_items)),
                    );
                    Value::Object(map)
                })
                .collect();
            let obj = Value::Array(list_response);
            (StatusCode::OK, Json(obj))
        }
        Err(err) => {
            eprintln!("Error fetching items: {}", err);
            let error_response = serde_json::json!({ "error": err.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

#[derive(Serialize)]
struct Item {
    item_id: Uuid,
    item_name: String,
    nb_of_items: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use serde_json::Value;
    use tower::ServiceExt; // Import to_bytes function

    #[tokio::test]
    async fn test_root() {
        let app = Router::new().route("/", get(root));

        let response = app.oneshot(Request::new(Body::empty())).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        assert_eq!(body, "Hello, World!");
    }

    #[tokio::test]
    async fn test_get_items() {
        // Setup database connection and pool
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();

        let app = Router::new()
            .route("/items", get(get_items))
            .layer(Extension(pool));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/items")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), 10000)
            .await
            .unwrap();
        let items: Vec<Value> = serde_json::from_slice(&body).unwrap();

        for item in items {
            assert!(item.get("item_id").is_some());
            assert!(item.get("item_name").is_some());
            assert!(item.get("nb_of_items").is_some());
        }
    }
}
