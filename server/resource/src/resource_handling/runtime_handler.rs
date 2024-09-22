use crate::{
    get_inventory_items, get_inventory_places, get_items, get_places, health, login_request,
    resource_handling::json_api_handler::add_items,
};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tracing::instrument;

#[tokio::main]
#[instrument]
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    //FIXME: ? add a config for testing in isolation and a test as well
    //1. create a config for testing purpose
    //2. remove it and put it in the readme
    //3. add a test

    // just to generate hashed pass for test purposes
    // pub fn hash_pass(user_password: &str) -> String {
    //     hash(user_password, DEFAULT_COST).unwrap()
    // }
    // let test = "123";
    // let hash_test = hash_pass(test);
    // println!("{:?}", hash_test);

    let app = Router::new()
        .route("/", get(health))
        .route("/items", get(get_items))
        .route("/places", get(get_places))
        .route("/inventory/items", get(get_inventory_items))
        .route("/inventory/items", post(add_items))
        .route("/inventory/places", get(get_inventory_places))
        .route("/login", post(login_request))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
