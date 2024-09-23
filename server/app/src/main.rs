use domain::OrderService;
use resource::AxumServerAdapter;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (sender, receiver) = broadcast::channel(10);
    let order_service = OrderService::new();

    let axum_server = AxumServerAdapter::new(sender.clone(), order_service);

    axum_server.listen_and_start_server(receiver).await;
}
