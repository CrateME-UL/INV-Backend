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
use sqlx::postgres::PgPool;
// use uuid::Uuid;
use async_std::task;
use sqlx::prelude::*;

#[tokio::main(flavor = "current_thread")]
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
    
}

// async fn get_users_db() -> anyhow::Result<()> {
//     let pool = PgPool::connect("postgres://postgres:mysecretpassword@127.0.0.1:5432/postgres").await.unwrap();
//     let _ = sqlx::query("SELECT * FROM users")
//         .fetch(&pool);
//     Ok(())
// }
async fn get_users_db() -> anyhow::Result<Vec<User>> {
    println!("test: {}", "avant!");
    // let pool = PgPool::connect("postgres://postgres:mysecretpassword@jdbc:postgresql://127.0.0.1:5432/postgres").await?;
    //let pool = PgPool::connect("jdbc:postgresql:mysecretpassword//localhost:5432/postgres").await?;
    let (client, connection) = tokio_postgres::Config::new()
    .host("localhost")
    .user("postgres")
    .password("mysecretpassword")
    .dbname("postgres")
    .connect(NoTls)
    .await?;
    println!("test: {}", "connected!");
    let users = sqlx::query_as("SELECT id, username FROM users")
        .fetch_all(&connection).collect
        .await?;

    Ok(users.into_iter().map(|user: (i32, String)| User { id: user.0 as u64, username: user.1 }).collect())
}
async fn get_users() -> impl IntoResponse {
let response: Vec<User> = task::block_on(get_users_db()).unwrap();
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


let list_response: Vec<Value> = response
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
