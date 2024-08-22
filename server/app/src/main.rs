use axum::{
    routing::{get, post},
    Router,
};
//use bcrypt::{hash, DEFAULT_COST};
use resource::{
    get_inventory_items, get_inventory_places, get_items, get_places, health, login_request,
};

use tower_http::cors::CorsLayer;
use tracing::instrument;
#[tokio::main]
#[instrument]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

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
        .route("/inventory/places", get(get_inventory_places))
        .route("/login", post(login_request))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::body::Body;
//     use axum::http::{Request, StatusCode};
//     use tower::ServiceExt;

//     #[tokio::test]
//     async fn test_root() {
//         let app = Router::new().route("/", get(health));

//         let response = app.oneshot(Request::new(Body::empty())).await.unwrap();

//         assert_eq!(response.status(), StatusCode::OK);

//         let body = axum::body::to_bytes(response.into_body(), 1024)
//             .await
//             .unwrap();
//         assert_eq!(body, "Hello, World!");
//     }

//     #[tokio::test]
//     async fn test_get_items() {
//         dotenv().ok();
//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         let pool = PgPoolOptions::new()
//             .max_connections(5)
//             .connect(&database_url)
//             .await
//             .unwrap();

//         let app = Router::new()
//             .route("/items", get(get_items))
//             .layer(Extension(pool));

//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .uri("/items")
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         assert_eq!(response.status(), StatusCode::OK);
//     }

//     #[tokio::test]
//     async fn test_get_places() {
//         dotenv().ok();
//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         let pool = PgPoolOptions::new()
//             .max_connections(5)
//             .connect(&database_url)
//             .await
//             .unwrap();

//         let app = Router::new()
//             .route("/places", get(get_places))
//             .layer(Extension(pool));

//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .uri("/places")
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         assert_eq!(response.status(), StatusCode::OK);
//     }
// }
