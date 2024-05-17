// //! Run with
// //!
// //! ```not_rust
// //! cargo run -p example-readme
// //! ```

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

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
        // 'get_/places' goes to `get_places`
        // .route("/places", get(get_places))
        // 'get_/items' goes to `get_items`
        .route("/items", get(get_items))
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

async fn create_place(Json(payload): Json<CreatePlace>) -> impl IntoResponse {
    let place = Place {
        place_id: payload.place_id,
        place_name: payload.place_name,
        place_type: payload.place_type,
    };

    (StatusCode::CREATED, Json(place))
}


async fn create_item(Json(payload): Json<CreateItem>) -> impl IntoResponse {
    let item = Item {
        item_id: Uuid::new_v4(),
        place_id: payload.place_id,
        nb_of_items: payload.nb_of_items,
        item_name: payload.item_name, 
    };
    (StatusCode::CREATED, Json(item))
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

// async fn get_places_db() -> anyhow::Result<Vec<Place>> {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&database_url)
//         .await?;
    
//     let places = sqlx::query_as!(Place, "SELECT placeId as place_id, placeName as place_name, placeType as place_type FROM Places")
//     .fetch_all(&pool)
//     .await?;

//     Ok(places)
// }

async fn get_items_db() -> anyhow::Result<Vec<Item>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let items = sqlx::query_as!(Item, "SELECT itemId as item_id, placeId as place_id, nbOfItems as nb_of_items, itemName as item_name FROM Items")
        .fetch_all(&pool)
        .await?;

    Ok(items)
}


// async fn get_places() -> impl IntoResponse {
//     match get_places_db().await {
//         Ok(places_result) => {
//             let list_response: Vec<Value> = places_result
//                 .into_iter()
//                 .map(|place| {
//                     let mut map = Map::new();
//                     map.insert("placeId".to_string(), Value::String(place.place_id.to_string()));
//                     map.insert("placeName".to_string(), Value::String(place.place_name));
//                     map.insert("placeType".to_string(), Value::String(place.place_type));
//                     Value::Object(map)
//                 })
//                 .collect();

//             let obj = Value::Array(list_response);
//             (StatusCode::OK, Json(obj))
//         }
//         Err(err) => {
//             eprintln!("Error fetching places: {}", err);
//             let error_response = serde_json::json!({ "error": err.to_string() });
//             (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
//         }
//     }
// }



async fn get_items() -> impl IntoResponse {
    match get_items_db().await {
        Ok(items_result) => {
            let list_response: Vec<Value> = items_result
                .into_iter()
                .map(|item| {
                    let mut map = Map::new();
                    map.insert("item_id".to_string(), Value::String(item.item_id.to_string()));
                    map.insert("place_id".to_string(), Value::String(item.place_id.to_string()));
                    map.insert("nb_of_items".to_string(), Value::String(item.nb_of_items));
                    map.insert("item_name".to_string(), Value::String(item.item_name));
                    Value::Object(map)
                })
                .collect();

            let obj = Value::Array(list_response);
            (StatusCode::OK, Json(obj))
        }
        Err(err) => {
            eprintln!("Error fetching items: {}", err);
            let error_response = serde_json::json!({ "error": err.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}
async fn get_users_db() -> anyhow::Result<Vec<User>> {
    // load environment variables from.env file
    dotenv().ok();

    // get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let users = sqlx::query_as("SELECT id, username FROM users")
        .fetch_all(&pool) // Use the pool to get a connection that implements the Executor trait
        .await?;

    Ok(users
        .into_iter()
        .map(|user: (i32, String)| User {
            id: user.0 as u64,
            username: user.1,
        })
        .collect())
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
#[derive(Deserialize)]
struct CreatePlace {
    place_id: Uuid,
    place_name: String,
    place_type: String,
}

#[derive(Serialize)]
struct Place {
    place_id: Uuid,
    place_name: String,
    place_type: String,
}

#[derive(Deserialize)]
struct CreateItem {
    item_id: Uuid,
    place_id: Uuid,
    nb_of_items: String,
    item_name: String,
}

#[derive(Serialize)]
struct Item {
    item_id: Uuid,
    place_id: Uuid,
    nb_of_items: String,
    item_name: String,
}

// #[derive(sqlx::Type, Deserialize, Serialize)]
// #[sqlx(type_name = "placeTypeSelector")]
// #[serde(rename_all = "snake_case")]
// enum PlaceType {
//     Out,
//     In,
// }
