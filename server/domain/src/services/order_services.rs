use std::sync::Arc;

use crate::{events::events::OrderPlaced, InventoryItemFetchable};

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

    pub fn place_order(&self, order_id: u64, amount: f64) -> OrderPlaced {
        let items = self.order_service_repository.fetch_inventory_items();
        println!("{:?}", items);
        println!("Order placed: ID = {}, amount = {}", order_id, amount);
        OrderPlaced { order_id, amount }
    }
}
