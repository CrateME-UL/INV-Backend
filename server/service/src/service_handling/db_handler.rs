use axum::{extract::Query, Json};
use domain::{
    InventoryItem, InventoryItemQuery, InventoryItemRequest, InventoryPlaceQuery, ItemListDb,
};
use repository::{
    get_inventory_items_db, get_inventory_places_db, get_places_db, AddInventoryItems, FetchItems,
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
    handle_db_result(ItemListDb::fetch_items()).await
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

pub async fn add_items_service(
    payload: Json<InventoryItemRequest>,
) -> Result<Value, Box<dyn Error>> {
    let inventory_item = InventoryItemRequest {
        place_name: payload.place_name.clone(),
        item_name: payload.item_name.clone(),
        nb_of_items: payload.nb_of_items,
    };

    // Call the function without an unnecessary async block
    let db_call = <InventoryItem as AddInventoryItems>::add_inventory_items(inventory_item).await;

    // Handle the result directly
    match db_call {
        Ok(result) => Ok(map_to_value(result)),
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(err)
        }
    }
}
