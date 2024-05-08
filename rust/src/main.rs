//! Run with
//!
//! ```not_rust
//! cargo run -p example-readme
//! ```

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
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



// async fn create_user(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     Json(payload): Json<CreateUser>,
// ) -> (StatusCode, impl IntoResponse) {
//     let userErr = User {
//         id: 0,
//         username: String::from(&payload.username),
//         };
//     // Insertion des données dans la base de données
//     match insert_user(&payload.username).await {
//         Ok(user) => (StatusCode::CREATED, Json(user)),
//         Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(userErr)),
//     }
// }

// // Fonction pour insérer un utilisateur dans la base de données
// async fn insert_user(username: &str) -> Result<User, sqlx::Error> {
//     // Création d'une connexion à la base de données
//     let pool = get_database_pool().await?;
    
    // Exécution de la requête SQL pour insérer l'utilisateur
//     let user = sqlx::query_as!(
//         User,
//         r#"INSERT INTO users (username) VALUES ($1) RETURNING id, username"#,
//         username
//     )
//     .fetch_one(&pool)
//     .await?;
    
//     Ok(user)
// }

// Fonction pour récupérer le pool de connexion à la base de données
// async fn get_database_pool() -> Result<sqlx::PgPool, sqlx::Error> {
//     // URL de connexion à la base de données PostgreSQL
//     let database_url = "postgresql://username:password@localhost/database";
    
//     // Création du pool de connexion
//     let pool = sqlx::PgPool::connect(database_url).await?;
    
//     Ok(pool)
// }



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
