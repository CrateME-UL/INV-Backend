//! TODO: remove mappers from resource and put them in service
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Extension, Json};
use repository::{get_inventory_items_db, get_inventory_places_db, get_items_db, get_places_db};
use serde::Deserialize;
use serde_json::{Map, Number, Value};
use sqlx::PgPool;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Deserialize, Debug)]
struct InventoryPlaceQuery {
    item_name: Option<String>,
}
#[derive(Deserialize, Debug)]
struct InventoryItemQuery {
    place_name: Option<String>,
    place_type: Option<String>,
}

async fn health() -> &'static str {
    "Hello, World!"
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
    query: Query<InventoryItemQuery>,
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
    query: Query<InventoryPlaceQuery>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
