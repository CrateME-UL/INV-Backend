use std::{collections::HashMap, error::Error, future::Future, pin::Pin};

use domain::{DomainError, Item, ItemNo, ItemRepository};

#[derive(Debug)]
pub struct InMemoryItemRepository {
    item_list: HashMap<String, Item>,
}

impl InMemoryItemRepository {
    pub fn new(item_list: HashMap<String, Item>) -> Self {
        Self { item_list }
    }
}

impl ItemRepository for InMemoryItemRepository {
    fn fetch_item_by_name(
        &self,
        item_name: String,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Option<Item>, Box<dyn Error>>> + Send>>
    {
        todo!("missing the tests to implement before the code ;P")
        // let item_list = self.item_list.clone();
        // Box::pin(async move { Ok(item_list.get(&item_name).cloned()) })
    }

    fn store_item(
        &self,
        item: Item,
    ) -> Pin<Box<(dyn Future<Output = Result<ItemNo, Box<dyn Error>>> + Send)>> {
        todo!()
    }
}
