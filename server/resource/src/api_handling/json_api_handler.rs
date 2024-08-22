use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use domain::{InventoryItemQuery, InventoryPlaceQuery, LoginRequest};
use serde_json::Value;
use service::{
    get_inventory_items_service, get_inventory_places_service, get_items_service,
    get_places_service, login_service,
};

pub async fn health() -> &'static str {
    "Hello, World!"
}

fn handle_service_result(
    result: Result<serde_json::Value, Box<dyn std::error::Error>>,
) -> impl IntoResponse {
    match result {
        Ok(data) => (StatusCode::OK, Json(data)),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": err.to_string() })),
        ),
    }
}

fn handle_service_result_login(
    result: Result<Value, Box<dyn std::error::Error>>,
) -> impl IntoResponse {
    match result {
        Ok(data) => (StatusCode::ACCEPTED, Json(data)),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": err.to_string() })),
        ),
    }
}

pub async fn get_items() -> impl IntoResponse {
    handle_service_result(get_items_service().await)
}

pub async fn get_places() -> impl IntoResponse {
    handle_service_result(get_places_service().await)
}

pub async fn get_inventory_items(query: Query<InventoryItemQuery>) -> impl IntoResponse {
    handle_service_result(get_inventory_items_service(query).await)
}

pub async fn get_inventory_places(query: Query<InventoryPlaceQuery>) -> impl IntoResponse {
    handle_service_result(get_inventory_places_service(query).await)
}

pub async fn login_request(payload: Json<LoginRequest>) -> impl IntoResponse {
    handle_service_result_login(login_service(payload).await)
}
