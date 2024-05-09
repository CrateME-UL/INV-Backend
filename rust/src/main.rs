//! Run with
//!
//! ```not_rust
//! cargo run -p example-readme
//! ```

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    // extract::Path,
    // extract::State,
    Json, Router,
};

// use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
// use uuid::Uuid;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `GET /users` goes to `user_id`
        .route("/users", get(get_users))	
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5432")
        .await
        .unwrap();
    println!("server running: {}", "127.0.0.1:5432");
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
    
}async fn get_users() -> impl IntoResponse {
    // insert your application logic here
let users: Vec<User> = vec![
    User {
        id: 0,
        username: String::from("test0"),
    },
    User {
        id: 1,
        username: String::from("test1"),
    },
];

let list_response: Vec<Value> = users
   .into_iter()
   .map(|user| {
        let mut map = Map::new();
        map.insert("id".to_string(), Value::Number(Number::from(user.id)));
        map.insert("username".to_string(), Value::String(user.username));
        Value::Object(map)
    })
   .collect();

let obj = Value::Array(list_response);




    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(obj))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
