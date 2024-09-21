use axum::extract::Query;

use domain::{InventoryItemQuery, InventoryPlaceQuery, ItemListDb};
use repository::{get_inventory_items_db, get_inventory_places_db, get_places_db, FetchItems};
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
