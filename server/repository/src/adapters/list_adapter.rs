use domain::{InventoryItem, InventoryItemFetchable};
use std::error::Error;
pub struct InMemoryListRepository;

impl InventoryItemFetchable for InMemoryListRepository {
    fn fetch_inventory_items(&self) -> Result<Vec<domain::InventoryItem>, Box<dyn Error>> {
        Ok(vec![InventoryItem {
            id: 1,
            object: "allo".to_string(),
            quantity: 2,
        }])
    }
}
