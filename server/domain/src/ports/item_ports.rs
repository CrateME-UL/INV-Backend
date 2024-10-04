use std::{error::Error, future::Future, pin::Pin};

use crate::models::item::Item;
pub trait ItemRepository: Send + Sync {
    fn fetch_item_by_id(
        &self,
        id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Item>, Box<dyn Error>>> + Send>>;
}
