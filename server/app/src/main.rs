use domain::{Inventory, Item, ItemRepository};
use repository::InMemoryItemRepository;
use resource::AxumServerAdapter;

use std::{collections::HashMap, sync::Arc};

fn main() {
    let inventory_list = HashMap::<String, Item>::new();
    let inventory_repository: Arc<dyn ItemRepository> =
        Arc::new(InMemoryItemRepository::new(inventory_list));
    let inventory = Inventory::new(inventory_repository);

    let axum_server = AxumServerAdapter::new(inventory);
    axum_server.listen_and_start_server();
}
