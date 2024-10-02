use std::error::Error;

use crate::models::item::Item;

pub trait ItemFetchable {
    async fn fetch_item_by_id(&self, id: i32) -> Result<Item, Box<dyn Error>>;
}

pub trait ItemRepository: ItemFetchable + Send + Sync {}

impl<T> ItemRepository for T where T: ItemFetchable  + Send + Sync {}
