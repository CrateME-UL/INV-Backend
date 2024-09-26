use std::sync::Arc;

use domain::OrderService;
use repository::adapters::list_adapter::InMemoryListRepository;
use resource::AxumServerAdapter;

#[tokio::main]
async fn main() {
    let inventory_fetcher = Arc::new(InMemoryListRepository);
    let order_service = OrderService::new(inventory_fetcher);

    let axum_server = AxumServerAdapter::new(order_service);

    axum_server.listen_and_start_server().await;
}
