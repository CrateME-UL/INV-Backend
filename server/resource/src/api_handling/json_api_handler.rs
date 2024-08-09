use axum::{extract::Query, http::StatusCode, response::IntoResponse, Extension, Json};
use domain::{
    CreateUser, InventoryItemQuery, InventoryPlaceQuery, LoginRequest, LoginResponse, User,
};
use serde_json::Value;
use service::{
    get_inventory_items_service,
    get_inventory_places_service,
    get_items_service,
    get_places_service,
    login_service, //get_users_service, // login_handler, post_user_service
                   //post_users_service,
};
use warp::reply::Reply;
//use sqlx::PgPool;
use std::error::Error;

pub async fn health() -> &'static str {
    "Hello, World!"
}

fn handle_service_result(
    result: Result<serde_json::Value, Box<dyn std::error::Error>>,
) -> impl IntoResponse {
    println!("{}:200", format!("{:?}:200", result));
    match result {
        Ok(data) => (StatusCode::OK, Json(data)),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": err.to_string() })),
        ),
    }
}
fn handle_service_result_created(
    result: Result<serde_json::Value, Box<dyn std::error::Error>>,
) -> impl IntoResponse {
    //println!("{}:204", format!("{:?}:204", result));
    //println!("{}:204-stat", format!("{:?}:204-stat", StatusCode::CREATED));
    match result {
        Ok(data) => (StatusCode::CREATED, Json(data)),
        Err(err) => {
            println!("{}:404", format!("{:?}:404", err));
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": err.to_string() })),
            )
        }
    }
}

fn handle_service_result_login(
    result: Result<Value, Box<dyn std::error::Error>>,
) -> impl IntoResponse {
    match result {
        Ok(data) => (StatusCode::ACCEPTED, Json(data)),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": err.to_string() })),
        ),
    }
}

pub async fn get_items() -> impl IntoResponse {
    handle_service_result(get_items_service().await)
}

pub async fn get_places() -> impl IntoResponse {
    handle_service_result(get_places_service().await)
}

pub async fn get_inventory_items(query: Query<InventoryItemQuery>) -> impl IntoResponse {
    handle_service_result(get_inventory_items_service(query).await)
}

pub async fn get_inventory_places(query: Query<InventoryPlaceQuery>) -> impl IntoResponse {
    handle_service_result(get_inventory_places_service(query).await)
}

pub async fn login_request(payload: Json<LoginRequest>) -> impl IntoResponse {
    let result = login_service(payload).await;
    print!("{:?}", result);
    handle_service_result_login(result)
}

// pub async fn create_user(payload: Json<CreateUser>) -> impl IntoResponse {
//     handle_service_result_created(post_users_service(payload).await);
// }

// pub async fn login_service(
//     Json(login_request): Json<LoginRequest>,
//     //Extension(pool): Extension<PgPool>,
// ) -> Result<impl IntoResponse, Box<dyn Error>> {
//     // Create a JSON payload for the login handler
//     let json_payload = Json(login_request);

//     // Call the login handler function
//     let response = login_handler(json_payload).await;

//     // Handle the result
//     match response {
//         Ok(response) => Ok((StatusCode::OK, response)), // Return a tuple with the status code and the response
//         Err(_) => Err("WrongCredentialsError".into()),  // Convert the error into a Box<dyn Error>
//     }
// }

// pub async fn login_service(query: Query<LoginRequest>) -> impl IntoResponse {

//     println!("{}: loginService", format!("{:?}", query));
//     handle_service_result(get_users_service(query).await)
// }
