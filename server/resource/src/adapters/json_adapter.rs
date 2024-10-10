use std::error::Error;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::AxumServerAdapter;

#[derive(Deserialize, Debug)]
pub struct InventoryPlaceQuery {
    pub _item_name: Option<String>,
    pub _place_type: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct InventoryItemQuery {
    pub _place_name: Option<String>,
    pub _place_type: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub _user_email: Option<String>,
    pub _user_password: Option<String>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct InventoryItemRequest {
    pub place_name: String,
    pub item_name: String,
    pub nb_of_items: i32,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ItemRequest {
    pub item_name: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Response {
    pub message: String,
}

fn _map_to_response(result: Result<Value, Box<dyn Error>>) -> impl IntoResponse {
    match result {
        Ok(data) => (StatusCode::OK, Json(data)),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": err.to_string() })),
        ),
    }
}

fn _map_to_value<T>(items: T) -> Value
where
    T: Serialize,
{
    serde_json::to_value(items).unwrap()
}

async fn _handle_future_result<T, F>(db_call: F) -> Result<Value, Box<dyn Error>>
where
    F: std::future::Future<Output = Result<T, Box<dyn Error>>>,
    T: Serialize,
{
    match db_call.await {
        Ok(result) => Ok(_map_to_value(result)),
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(err)
        }
    }
}

// #[axum::debug_handler]
// pub async fn add_item(
//     State(adapter): State<AxumServerAdapter>,
//     Json(payload): Json<ItemRequest>,
// ) -> impl IntoResponse {
//     let inventory_item = Item {
//         item_id: None,
//     };

//     let result = adapter
//         .inventory_item_service
//         .add_inventory_items(inventory_item)
//         .await;

//     match result {
//         Ok(item) => {
//             let response = Response {
//                 message: format!(
//                     "Successfully added item: {}",
//                     item.item_name.unwrap_or("does not exist".to_string())
//                 ),
//             };
//             (StatusCode::CREATED, Json(response))
//         }
//         Err(err) => {
//             eprintln!("Error adding item: {}", err);
//             let response = Response {
//                 message: format!("Error adding item: {}", err),
//             };
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
//         }
//     }
// }

// #[axum::debug_handler]
// pub async fn add_inventory_item(
//     State(adapter): State<AxumServerAdapter>,
//     Json(payload): Json<InventoryItemRequest>,
// ) -> impl IntoResponse {
//     let inventory_item = InventoryItem {
//         item_id: None,
//         place_name: payload.place_name.clone(),
//         item_name: Some(payload.item_name.clone()),
//         place_type: None,
//         nb_of_items: payload.nb_of_items,
//     };

//     let result = adapter
//         .inventory_item_service
//         .add_inventory_items(inventory_item)
//         .await;

//     match result {
//         Ok(item) => {
//             let response = Response {
//                 message: format!(
//                     "Successfully added item: {}",
//                     item.item_name.unwrap_or("does not exist".to_string())
//                 ),
//             };
//             (StatusCode::CREATED, Json(response))
//         }
//         Err(err) => {
//             eprintln!("Error adding item: {}", err);
//             let response = Response {
//                 message: format!("Error adding item: {}", err),
//             };
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
//         }
//     }
// }

// pub async fn health() -> &'static str {
//     "Hello, World!"
// }

// #[axum::debug_handler]
// pub async fn get_items(State(adapter): State<AxumServerAdapter>) -> impl IntoResponse {

//     let result = adapter
//     .item_service
//     .fetch_item_by_name(inventory_item)
//     .await;
//     match result {
//         Ok(item) => {
//             let response = Response {
//                 message: format!(
//                     "Successfully added item: {}",
//                     item.item_name.unwrap_or("does not exist".to_string())
//                 ),
//             };
//             (StatusCode::CREATED, Json(response))
//         }
//         Err(err) => {
//             eprintln!("Error adding item: {}", err);
//             let response = Response {
//                 message: format!("Error adding item: {}", err),
//             };
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
//         }
//     }
// }

// pub async fn get_places() -> impl IntoResponse {
//     todo!();
// }

// pub async fn get_inventory_items(
//     State(adapter): State<AxumServerAdapter>,
//     query: Query<InventoryItemQuery>,
// ) -> impl IntoResponse {
//     todo!();
//     // let default = "";
//     // let inventory_item = InventoryItem {
//     //     item_id: None,
//     //     place_name: query.place_name.as_deref().unwrap_or(default).to_string(),
//     //     item_name: None,
//     //     place_type: query.place_type,
//     //     nb_of_items: -1,
//     // };

//     // let result = adapter.inventory_item_service
//     //     .fetch_inventory_items(inventory_item)
//     //     .await;
// }

// pub async fn get_inventory_places(_query: Query<InventoryPlaceQuery>) -> impl IntoResponse {
//     todo!();
// }

// pub async fn login_request(_payload: Json<LoginRequest>) -> impl IntoResponse {
//     todo!();
// }
