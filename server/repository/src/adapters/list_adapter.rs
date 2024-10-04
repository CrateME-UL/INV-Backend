use std::error::Error;

use domain::{Item, ItemRepository};

#[derive(Debug)]
pub struct InMemoryItemRepository;

impl ItemRepository for InMemoryItemRepository {
    fn fetch_item_by_id(
        &self,
        id: i32,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Option<Item>, Box<dyn Error>>> + Send>,
    > {
        todo!()
    }
}
