use std::sync::Arc;

use domain::OrderService;
use repository::in_memory_handler::list_handler::InMemoryListRepository;
use resource::AxumServerAdapter;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (sender, receiver) = broadcast::channel(10);
    let inventory_fetcher = Arc::new(InMemoryListRepository);
    let order_service = OrderService::new(inventory_fetcher);

    let axum_server = AxumServerAdapter::new(sender.clone(), order_service);

    axum_server.listen_and_start_server(receiver).await;
}
