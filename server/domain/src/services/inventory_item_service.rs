use std::{error::Error, sync::Arc};

use crate::{InventoryItem, InventoryItemFetchable};

#[derive(Clone)]
pub struct OrderService {
    order_service_repository: Arc<dyn InventoryItemFetchable + Send + Sync>,
}

impl OrderService {
    pub fn new(order_service_repository: Arc<dyn InventoryItemFetchable + Send + Sync>) -> Self {
        Self {
            order_service_repository,
        }
    }

    pub fn fetch_inventory_items(&self) -> Result<Vec<InventoryItem>, Box<dyn Error>> {
        let items = self.order_service_repository.fetch_inventory_items();
        println!("{:?}", items);
        items
    }
}
