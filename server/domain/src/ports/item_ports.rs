use std::{error::Error, future::Future, pin::Pin};

use crate::{models::{item::Item, place::Place}, ItemNo};
pub trait ItemRepository: Send + Sync {
    fn fetch_item_by_name(
        &self,
        item_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, Box<dyn Error>>> + Send>>;

    fn fetch_place_by_name(
        &self,
        place_name: String,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Place>, Box<dyn Error>>> + Send>>;

    fn store_item(
        &self,
        item: Item,
    ) -> Pin<Box<dyn Future<Output = Result<ItemNo, Box<dyn Error>>> + Send>>;
}
