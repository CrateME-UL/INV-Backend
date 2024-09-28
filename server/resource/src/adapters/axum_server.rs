use axum::{
    routing::{get, post},
    Router,
};
use domain::InventoryItemService;
use tower_http::cors::CorsLayer;
// use tracing::instrument;

use crate::adapters::json_adapter::{
    add_inventory_item, get_inventory_places, get_items, get_places, health, login_request,
};

#[derive(Clone)]
pub struct AxumServerAdapter {
    pub inventory_item_service: InventoryItemService,
}

impl AxumServerAdapter {
    pub fn new(order_service: InventoryItemService) -> Self {
        Self {
            inventory_item_service: order_service,
        }
    }
    #[tokio::main]
    // #[instrument]
    pub async fn listen_and_start_server(self) {
        self.start_server().await;
    }

    async fn start_server(&self) {
        let app = Router::new()
            .route("/", get(health))
            .route("/items", get(get_items))
            .route("/places", get(get_places))
            // .route("/inventory/items", get(get_inventory_items))
            .route("/inventory/items", post(add_inventory_item))
            .route("/inventory/places", get(get_inventory_places))
            .route("/login", post(login_request))
            .layer(CorsLayer::permissive())
            .with_state(self.clone());

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        // tracing::debug!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
    }
}
