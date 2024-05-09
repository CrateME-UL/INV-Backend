// //! Run with
// //!
// //! ```not_rust
// //! cargo run -p example-readme
// //! ```


use axum::{
    http::StatusCode,
     response::IntoResponse,
     routing::{get, post},
     Json, 
     Router,
 };

 use serde::{Deserialize, Serialize};
 use serde_json::{Map, Number, Value};
 use sqlx::postgres::PgPoolOptions;
 use tower_http::cors::CorsLayer;
 use dotenv::dotenv;
use std::env;


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
         .route("/users", post(create_user))
         //enable cors
         .layer(CorsLayer::permissive());

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



async fn get_users_db() -> anyhow::Result<Vec<User>> {
     // load environment variables from.env file
     dotenv().ok();

     // get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Create a connection pool
    let pool = PgPoolOptions::new()
       .max_connections(5)
       .connect(&database_url).await?;

    let users = sqlx::query_as("SELECT id, username FROM users")
       .fetch_all(&pool) // Use the pool to get a connection that implements the Executor trait
       .await?;

    Ok(users.into_iter().map(|user: (i32, String)| User { id: user.0 as u64, username: user.1 }).collect())
}

async fn get_users() -> impl IntoResponse {
    match get_users_db().await {
        Ok(users_result) => {
            let list_response: Vec<Value> = users_result
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
        Err(err) => {
            // Handle the error and return a default response
            eprintln!("Error fetching users: {}", err);
            let error_response = serde_json::json!({ "error": err.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
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
