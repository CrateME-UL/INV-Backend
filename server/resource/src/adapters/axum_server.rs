use axum::Router;
use domain::ItemService;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AxumServerAdapter {
    pub item_service: ItemService,
}

impl AxumServerAdapter {
    pub fn new(item_service: ItemService) -> Self {
        Self { item_service }
    }
    #[tokio::main]
    pub async fn listen_and_start_server(self) {
        self.start_server().await;
    }

    async fn start_server(&self) {
        let app = Router::new()
            // .route("/", get(health))
            // .route("/items", get(get_items))
            // .route("/places", get(get_places))
            // .route("/inventory/items", get(get_inventory_items))
            // .route("/inventory/items", post(add_inventory_item))
            // .route("/inventory/places", get(get_inventory_places))
            // .route("/login", post(login_request))
            .layer(CorsLayer::permissive())
            .with_state(self.clone());

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
