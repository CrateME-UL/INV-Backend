use std::error::Error;

use domain::{Item, ItemRepository, Place};

#[derive(Debug)]
pub struct InMemoryItemRepository;

impl ItemRepository for InMemoryItemRepository {
    fn fetch_item_by_name(
        &self,
        item_name: String,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Option<Item>, Box<dyn Error>>> + Send>,
    > {
        todo!()
    }
    
    fn fetch_place_by_name(
        &self,
        place_name: String,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Option<Place>, Box<dyn Error>>> + Send>> {
        todo!()
    }
}
