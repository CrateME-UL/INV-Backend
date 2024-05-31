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

use excel_reader::read_excel;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::CorsLayer;
use tracing::instrument;

//TODO: add places implementation (to show places instead of place id)
//TODO: refactor code
//TODO: add tests

#[derive(Debug, Deserialize)]
struct Data {
    place: String,
    obj: String,
    qte: i32,
    emp: String,
}

#[tokio::main(flavor = "current_thread")]
#[instrument]
async fn main() {

    match read_excel::<Data>("../inventaire-mapping.csv") {
        Ok(records) => {
            for record in records {
                println!("{:?}; {:?}; {:?}; {:?}; ", record.place, record.obj, record.qte, record.emp);
            }
        }
        Err(e) => eprintln!("Error reading CSV file: {}", e),
    }
    // initialize tracing
    dotenv().ok();
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
        .route("/places", get(get_places))
        // 'get_/items' goes to `get_items`
        .route("/items", get(get_items))
        //enable cors
        .layer(CorsLayer::permissive());

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// async fn create_place(Json(payload): Json<CreatePlace>) -> impl IntoResponse {
//     let place = Place {
//         place_id: payload.place_id,
//         place_name: payload.place_name,
//         place_type: payload.place_type,
//     };

//     (StatusCode::CREATED, Json(place))
// }


// async fn create_item(Json(payload): Json<CreateItem>) -> impl IntoResponse {
//     let item = Item {
//         item_id: Uuid::new_v4(),
//         place_id: payload.place_id,
//         nb_of_items: payload.nb_of_items,
//         item_name: payload.item_name, 
//     };
//     (StatusCode::CREATED, Json(item))
// }

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

async fn get_places_db() -> anyhow::Result<Vec<Place>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    let places = sqlx::query_as("SELECT placeId, placeName, placeType FROM Places")
    .fetch_all(&pool)
    .await?;

    
    Ok(places
        .into_iter()
        .map(|place: (i32, String, String)| Place {
            place_id: place.0,
            place_name: place.1,
            place_type: place.2,
        })
        .collect())
}

async fn get_items_db() -> anyhow::Result<Vec<Item>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    let items = sqlx::query_as("SELECT itemId, placeId, nbOfItems, itemName FROM Items")
        .fetch_all(&pool)
        .await?;
    Ok(items
        .into_iter()
        .map(|item: (i32, i32, i32, String)| Item {
            item_id: item.0,
            place_id: item.1,
            nb_of_items: item.2,
            item_name: item.3,
        })
        .collect())

}


async fn get_places() -> impl IntoResponse {
    match get_places_db().await {
        Ok(places_result) => {
            let list_response: Vec<Value> = places_result
                .into_iter()
                .map(|place| {
                    let mut map = Map::new();
                    map.insert("placeId".to_string(), Value::Number(Number::from(place.place_id)));
                    map.insert("placeName".to_string(), Value::String(place.place_name));
                    map.insert("placeType".to_string(), Value::String(place.place_type));
                    Value::Object(map)
                })
                .collect();

            let obj = Value::Array(list_response);
            (StatusCode::OK, Json(obj))
        }
        Err(err) => {
            eprintln!("Error fetching places: {}", err);
            let error_response = serde_json::json!({ "error": err.to_string() });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}



async fn get_items() -> impl IntoResponse {
    match get_items_db().await {
        Ok(items_result) => {
            let list_response: Vec<Value> = items_result
                .into_iter()
                .map(|item| {
                    let mut map = Map::new();
                    map.insert("item_id".to_string(), Value::Number(Number::from(item.item_id)));
                    map.insert("place_id".to_string(), Value::Number(Number::from(item.place_id)));
                    map.insert("nb_of_items".to_string(), Value::Number(Number::from(item.nb_of_items)));
                    map.insert("item_name".to_string(), Value::String(item.item_name));
                    Value::Object(map)
                })
                .collect();
            let obj = Value::Array(list_response);
            (StatusCode::CREATED, Json(obj))
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
// #[derive(Deserialize)]
// struct CreatePlace {
//     place_id: u32,
//     place_name: String,
//     place_type: String,
// }

#[derive(Serialize)]
struct Place {
    place_id: i32,
    place_name: String,
    place_type: String,
}

// #[derive(Deserialize)]
// struct CreateItem {
//     item_id: u32,
//     place_id: u32,
//     nb_of_items: String,
//     item_name: String,
// }

#[derive(Serialize)]
struct Item {
    item_id: i32,
    place_id: i32,
    nb_of_items: i32,
    item_name: String,
}


