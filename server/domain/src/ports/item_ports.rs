use std::{error::Error, future::Future, pin::Pin};


pub trait ItemFetchable: Send + Sync {
    fn fetch_item_by_id(
        &self,
        id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<i32, Box<dyn Error>>> + Send>>;
}

pub trait ItemRepository: ItemFetchable + Send + Sync {}

impl<T> ItemRepository for T where T: ItemFetchable  + Send + Sync {}
