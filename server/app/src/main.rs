use domain::{Item, ItemRepository, ItemService};
use repository::InMemoryItemRepository;
use resource::AxumServerAdapter;

use std::{collections::HashMap, sync::Arc};

fn main() {
    let item_list = HashMap::<String, Item>::new();
    let item_repository: Arc<dyn ItemRepository> = Arc::new(InMemoryItemRepository::new(item_list));
    let item_service = ItemService::new(item_repository);

    let axum_server = AxumServerAdapter::new(item_service);
    axum_server.listen_and_start_server();
}
