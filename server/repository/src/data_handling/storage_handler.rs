use domain::{InventoryItem, InventoryItemRequest, Item};
pub trait FetchItems {
    fn fetch_items(
    ) -> impl std::future::Future<Output = Result<Vec<Item>, Box<dyn std::error::Error>>> + Send;
}
pub trait AddInventoryItems {
    fn add_inventory_items(
        inventory_item_list: InventoryItemRequest,
    ) -> impl std::future::Future<Output = Result<InventoryItem, Box<dyn std::error::Error>>> + Send;
}
