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
        .route("/inventory/items", get(get_inventory_items))
        .route("/inventory/places", get(get_inventory_places))
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
    let items = sqlx::query!("SELECT item_id, item_name FROM Items ORDER BY item_name;")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|record| Item {
            item_id: record.item_id,
            item_name: record.item_name,
        })
        .collect();

    Ok(items)
}

async fn get_places_db(pool: &PgPool) -> Result<Vec<Place>, Box<dyn std::error::Error>> {
    let places =
        sqlx::query!("SELECT place_id, place_name, place_type FROM Places ORDER BY place_name;")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|record| Place {
                place_id: record.place_id,
                place_name: record.place_name,
                place_type: record.place_type,
            })
            .collect();

    Ok(places)
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

async fn get_inventory_items(
    Extension(pool): Extension<PgPool>,
    query: Query<InventoryQuery>,
) -> impl IntoResponse {
    match get_inventory_items_db(&pool, &query).await {
        Ok(items_result) => {
            let list_response: Vec<Value> = items_result
                .into_iter()
                .map(|item| {
                    let mut map = Map::new();
                    map.insert(
                        "item_id".to_string(),
                        Value::Number(Number::from(item.item_id)),
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
            eprintln!("Error fetching inventory items: {}", err);
            let error_response = serde_json::json!({ "error": err.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

async fn get_inventory_places(
    Extension(pool): Extension<PgPool>,
    query: Query<InventoryQuery>,
) -> impl IntoResponse {
    match get_inventory_places_db(&pool, &query).await {
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
                    map.insert(
                        "nb_of_items".to_string(),
                        Value::Number(Number::from(place.nb_of_items)),
                    );
                    Value::Object(map)
                })
                .collect();
            let obj = Value::Array(list_response);
            (StatusCode::OK, Json(obj))
        }
        Err(err) => {
            eprintln!("Error fetching inventory places: {}", err);
            let error_response = serde_json::json!({ "error": err.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

async fn get_inventory_items_db(
    pool: &PgPool,
    query: &Query<InventoryQuery>,
) -> Result<Vec<InventoryItem>, Box<dyn std::error::Error>> {
    let items = sqlx::query!(
        "SELECT Items.item_id as item_id, Items.item_name as item_name, Inventory.nb_of_items as nb_of_items
            FROM Inventory
            JOIN Places ON Inventory.place_id = Places.place_id
            JOIN Items ON Inventory.item_id = Items.item_id
            WHERE (place_name =  $1 OR $1 = '' OR $1 = NULL) 
                AND (place_type = $2 OR $2 = '' OR $2 = NULL)
                AND (item_name = $3 OR $3 = '' OR $3 = NULL)
            ORDER BY Inventory.nb_of_items DESC;",
        query.place_name,
        query.place_type,
        query.item_name,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|record| InventoryItem {
        item_id: record.item_id,
        item_name: record.item_name,
        nb_of_items: record.nb_of_items,
    })
    .collect();

    Ok(items)
}

async fn get_inventory_places_db(
    pool: &PgPool,
    query: &Query<InventoryQuery>,
) -> Result<Vec<InventoryPlace>, Box<dyn std::error::Error>> {
    let places = sqlx::query!(
        "SELECT Places.place_id as place_id, Places.place_name as place_name, Places.place_type as place_type, Inventory.nb_of_items as nb_of_items
            FROM Inventory
            JOIN Places ON Inventory.place_id = Places.place_id
            JOIN Items ON Inventory.item_id = Items.item_id
            WHERE (place_name =  $1 OR $1 = '' OR $1 = NULL) 
                AND (place_type = $2 OR $2 = '' OR $2 = NULL)
                AND (item_name = $3 OR $3 = '' OR $3 = NULL)
            ORDER BY Inventory.nb_of_items DESC;",
        query.place_name,
        query.place_type,
        query.item_name,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|record| InventoryPlace {
        place_id: record.place_id,
        place_name: record.place_name,
        place_type: record.place_type,
        nb_of_items: record.nb_of_items,
    })
    .collect();

    Ok(places)
}

#[derive(Serialize)]
struct Item {
    item_id: i32,
    item_name: String,
}

#[derive(Serialize)]
struct Place {
    place_id: i32,
    place_name: String,
    place_type: String,
}

#[derive(Serialize)]
struct InventoryItem {
    item_id: i32,
    item_name: String,
    nb_of_items: i32,
}

#[derive(Serialize)]
struct InventoryPlace {
    place_id: i32,
    place_name: String,
    place_type: String,
    nb_of_items: i32,
}

#[derive(Deserialize)]
struct InventoryQuery {
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
