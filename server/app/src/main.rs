// //! Run with
// //!
// //! ```not_rust
// //! cargo run -p example-readme
// //! ```

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use uuid::Uuid;
use dotenv::dotenv;
use serde::Serialize;
use serde_json::{Map, Number, Value};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::CorsLayer;
use tracing::instrument;

//TODO: add places implementation (to show places instead of place id)
//TODO: refactor code
//TODO: add tests


// impl<'de> Deserialize<'de> for Data {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let value: Value = Deserialize::deserialize(deserializer)?;
//         let obj = value.get("obj").and_then(Value::as_str).unwrap_or_default().to_string();
//         let place = value.get("place").and_then(Value::as_str).unwrap_or_default().to_string();
//         let emp = value.get("emp").and_then(Value::as_str).unwrap_or_default().to_string();
//         let qte = value
//             .get("qte")
//             .and_then(Value::as_str)
//             .unwrap_or_default()
//             .parse::<i32>()
//             .map_err(serde::de::Error::custom)?;

//         Ok(Data { place, obj, qte, emp })
//     }
// }

#[tokio::main(flavor = "current_thread")]
#[instrument]
async fn main() {
    // initialize tracing
    dotenv().ok();
    tracing_subscriber::fmt::init();


    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
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
    let items = sqlx::query_as(
        "SELECT itemName, SUM(nbOfItems) as nbOfItems FROM Items GROUP BY itemName"
    )
    .fetch_all(&pool)
    .await?;
    Ok(items
        .into_iter()
        .map(|item: (String, i64)| Item {
            item_id: uuid::Uuid::new_v4(),
            item_name: item.0,
            nb_of_items: item.1 as i32,
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
                    map.insert("item_id".to_string(), Value::String(item.item_id.to_string()));
                    map.insert("item_name".to_string(), Value::String(item.item_name));
                    map.insert("nb_of_items".to_string(), Value::Number(Number::from(item.nb_of_items)));
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


// the input to our `create_user` handler

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
    item_id: Uuid,
    item_name: String,
    nb_of_items: i32,
}

