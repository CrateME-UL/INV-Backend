use std::error::Error;

use crate::InventoryItem;

pub trait InventoryItemFetchable {
    fn fetch_inventory_items(&self) -> Result<Vec<InventoryItem>, Box<dyn Error>>;
}
