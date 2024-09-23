use domain::{InventoryItem, InventoryItemRequest, Item};
pub trait FetchItems {
    fn fetch_items(
    ) -> impl std::future::Future<Output = Result<Vec<Item>, Box<dyn std::error::Error>>> + Send;
}
pub trait AddInventoryItem {
    fn add_inventory_item(
        inventory_item_list: InventoryItemRequest,
    ) -> impl std::future::Future<Output = Result<InventoryItem, Box<dyn std::error::Error>>> + Send;
}

pub trait InventoryRepository {
    fn find_place_id(
        &self,
        place_name: &str,
    ) -> impl std::future::Future<Output = Result<i32, Box<dyn std::error::Error>>>;
    fn find_item_id(
        &self,
        item_name: &str,
    ) -> impl std::future::Future<Output = Result<i32, Box<dyn std::error::Error>>>;
    fn inventory_exists(
        &self,
        place_id: i32,
        item_id: i32,
    ) -> impl std::future::Future<Output = Result<bool, Box<dyn std::error::Error>>> + Send;
    fn add_inventory(
        &self,
        place_id: i32,
        item_id: i32,
        nb_of_items: i32,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
    fn update_inventory(
        &self,
        place_id: i32,
        item_id: i32,
        nb_of_items: i32,
    ) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}
