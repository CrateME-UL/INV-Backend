use axum::extract::Query;
use axum::Json;
use domain::{CreateUser, InventoryItemQuery, InventoryPlaceQuery, LoginRequest, User};
use repository::{
    get_inventory_items_db,
    get_inventory_places_db,
    get_items_db,
    get_places_db,
    //post_user_db, //get_user_db,
};
use serde::Serialize;
use serde_json::Value;

use std::error::Error;

fn map_to_value<T>(items: T) -> Value
where
    T: Serialize,
{
    serde_json::to_value(items).unwrap()
}

async fn handle_db_result<T, F>(db_call: F) -> Result<Value, Box<dyn Error>>
where
    F: std::future::Future<Output = Result<T, Box<dyn Error>>>,
    T: Serialize,
{
    match db_call.await {
        Ok(result) => Ok(map_to_value(result)),
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(err)
        }
    }
}

pub async fn get_items_service() -> Result<Value, Box<dyn Error>> {
    handle_db_result(get_items_db()).await
}

pub async fn get_places_service() -> Result<Value, Box<dyn Error>> {
    handle_db_result(get_places_db()).await
}

pub async fn get_inventory_items_service(
    query: Query<InventoryItemQuery>,
) -> Result<Value, Box<dyn Error>> {
    handle_db_result(get_inventory_items_db(&query)).await
}

pub async fn get_inventory_places_service(
    query: Query<InventoryPlaceQuery>,
) -> Result<Value, Box<dyn Error>> {
    handle_db_result(get_inventory_places_db(&query)).await
}

// pub async fn get_users_service(query: Query<LoginRequest>) -> Result<Value, Box<dyn Error>> {
//     handle_db_result(get_user_db(&query)).await
// }
// pub async fn post_users_service(payload: Json<CreateUser>) -> Result<Value, Box<dyn Error>> {
//     handle_db_result(post_user_db(payload)).await
// }

// pub async fn get_users_service(payload: Json<CreateUser>) -> Result<Value, Box<dyn Error>> {

//     let user = User {
//         id: 1337,
//         username: payload.username.clone(),
//     };
//     handle_db_result(User)
// }
