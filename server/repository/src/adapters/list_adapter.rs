use domain::{InventoryItem, InventoryItemFetchable};
use std::error::Error;

#[derive(Debug)]
pub struct InMemoryListRepository;

impl InventoryItemFetchable for InMemoryListRepository {
    fn add_inventory_items(
        &self,
        _inventory_item: InventoryItem,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<InventoryItem, Box<dyn Error>>> + Send>,
    > {
        todo!()
    }

    fn fetch_inventory_items(
        &self,
        inventory_item: InventoryItem,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<InventoryItem>, Box<dyn Error>>> + Send>,
    > {
        todo!()
    }
}
