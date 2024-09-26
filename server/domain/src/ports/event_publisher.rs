use std::error::Error;

use crate::{events::events::OrderPlaced, InventoryItems};

pub trait EventPublisherPort {
    fn process_order_placed(&self, event: OrderPlaced);
}

pub trait InventoryItemFetchable {
    fn fetch_inventory_items(&self) -> Result<Vec<InventoryItems>, Box<dyn Error>>;
}
