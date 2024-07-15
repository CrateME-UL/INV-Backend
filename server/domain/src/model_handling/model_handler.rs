use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Item {
    pub item_id: i32,
    pub item_name: String,
}

#[derive(Serialize)]
pub struct Place {
    pub place_id: i32,
    pub place_name: String,
    pub place_type: String,
}

#[derive(Serialize)]
pub struct InventoryItem {
    pub item_id: i32,
    pub item_name: String,
    pub nb_of_items: i32,
}

#[derive(Serialize, Debug)]
pub struct InventoryPlace {
    pub place_id: i32,
    pub place_name: String,
    pub place_type: String,
    pub nb_of_items: i32,
}

#[derive(Deserialize, Debug)]
pub struct InventoryPlaceQuery {
    pub item_name: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct InventoryItemQuery {
    pub place_name: Option<String>,
    pub place_type: Option<String>,
}
