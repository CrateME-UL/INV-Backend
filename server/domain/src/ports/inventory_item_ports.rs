// use crate::InventoryItem;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

pub trait InventoryItemFetchable: Send + Sync {
    // fn fetch_inventory_items(
    //     &self,
    //     inventory_item: InventoryItem,
    // ) -> Pin<Box<dyn Future<Output = Result<Vec<InventoryItem>, Box<dyn Error>>> + Send>>;

    // fn add_inventory_items(
    //     &self,
    //     inventory_item: InventoryItem,
    // ) -> Pin<Box<dyn Future<Output = Result<InventoryItem, Box<dyn Error>>> + Send>>;
}
