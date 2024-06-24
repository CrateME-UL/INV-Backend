use axum::{
    extract::Query, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use tower_http::cors::CorsLayer;
use tracing::instrument;
use uuid::Uuid;

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
        .route("/places", get(get_places))
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

async fn get_items_db(
    pool: &PgPool,
    query: &Query<ItemQuery>,
) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let items = sqlx::query!(
        "SELECT itemName, SUM(nbOfItems) AS nbOfItems
        FROM Items
        JOIN Places ON Items.placeId = Places.placeId
        WHERE (Places.placeName = $1 OR $1 IS NULL or $1='') 
            AND (Places.placeType = $2 OR $2 IS NULL or $2='') 
            AND (Items.itemName = $3 OR $3 IS NULL or $3='')
        GROUP BY itemName
        ORDER BY nbOfItems DESC;",
        query.place_name,
        query.place_type,
        query.item_name,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|record| Item {
        item_id: Uuid::new_v4(),
        item_name: record.itemname,
        nb_of_items: record.nbofitems.unwrap_or_default() as i32,
    })
    .collect();

    Ok(items)
}
async fn get_places_db(pool: &PgPool) -> Result<Vec<Place>, Box<dyn std::error::Error>> {
    let places =
        sqlx::query!("SELECT placeId, placeName, placeType FROM Places ORDER BY placeName;")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|record| Place {
                place_id: record.placeid,
                place_name: record.placename,
                place_type: record.placetype,
            })
            .collect();

    Ok(places)
}

async fn get_items(
    Extension(pool): Extension<PgPool>,
    query: Query<ItemQuery>,
) -> impl IntoResponse {
    match get_items_db(&pool, &query).await {
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

async fn get_places(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    match get_places_db(&pool).await {
        Ok(places_result) => {
            let list_response: Vec<Value> = places_result
                .into_iter()
                .map(|place| {
                    let mut map = Map::new();
                    map.insert(
                        "place_id".to_string(),
                        Value::Number(Number::from(place.place_id)),
                    );
                    map.insert("place_name".to_string(), Value::String(place.place_name));
                    map.insert("place_type".to_string(), Value::String(place.place_type));
                    Value::Object(map)
                })
                .collect();
            let obj = Value::Array(list_response);
            (StatusCode::OK, Json(obj))
        }
        Err(err) => {
            eprintln!("Error fetching places: {}", err);
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
#[derive(Serialize)]
struct Place {
    place_id: i32,
    place_name: String,
    place_type: String,
}
#[derive(Deserialize)]
struct ItemQuery {
    place_name: Option<String>,
    place_type: Option<String>,
    item_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

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
    }

    #[tokio::test]
    async fn test_get_places() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();

        let app = Router::new()
            .route("/places", get(get_places))
            .layer(Extension(pool));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/places")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
