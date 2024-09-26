use domain::{InventoryItemFetchable, InventoryItems};
use std::error::Error;
pub struct InMemoryListRepository;

impl InventoryItemFetchable for InMemoryListRepository {
    fn fetch_inventory_items(&self) -> Result<Vec<domain::InventoryItems>, Box<dyn Error>> {
        Ok(vec![InventoryItems {
            id: 1,
            object: "allo".to_string(),
            quantity: 2,
        }])
    }
}
