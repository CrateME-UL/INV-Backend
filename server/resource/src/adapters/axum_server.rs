use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use domain::OrderService;

#[derive(Clone)]
pub struct AxumServerAdapter {
    pub order_service: OrderService,
}

impl AxumServerAdapter {
    pub fn new(order_service: OrderService) -> Self {
        Self { order_service }
    }

    pub async fn listen_and_start_server(self) {
        self.start_server().await;
    }

    async fn start_server(&self) {
        let app = Router::new()
            .route("/", get(health))
            .route("/order", post(place_order))
            .with_state(self.clone());

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

//TODO: place the methods in a different file
//TODO: in a different file add a mapper (json to domain) and (domain to json)
async fn place_order(State(axum_server): State<AxumServerAdapter>) -> Json<&'static str> {
    let _ = axum_server.order_service.fetch_inventory_items();
    Json("Order placed")
}

pub async fn health() -> &'static str {
    "Hello, World!"
}
