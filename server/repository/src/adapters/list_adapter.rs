use std::{collections::HashMap, error::Error};

use domain::{Item, ItemRepository, Place};

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
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Option<Item>, Box<dyn std::error::Error>>>
                + Send,
        >,
    > {
        todo!("missing the tests to implement before the code ;P")
        // let item_list = self.item_list.clone();
        // Box::pin(async move { Ok(item_list.get(&item_name).cloned()) })
    }

    fn fetch_place_by_name(
        &self,
        place_name: String,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Option<Place>, Box<dyn Error>>> + Send>,
    > {
        todo!()
    }
    
    fn store_item(
        &self,
        item: Item,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<domain::ItemNo, Box<dyn Error>>> + Send>> {
        todo!()
    }
    

}
