use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct Item {
    pub item_id: i32,
    pub item_name: String,
}

#[derive(Serialize, Debug)]
pub struct Place {
    pub place_id: i32,
    pub place_name: String,
    pub place_type: String,
}

#[derive(Serialize, Debug)]
pub struct InventoryItem {
    pub item_id: i32,
    pub item_name: String,
    pub nb_of_items: i32,
}

#[derive(Serialize, Debug)]
pub struct InventoryPlace {
    pub place_id: i32,
    pub place_name: String,
    pub place_type: String,
    pub nb_of_items: i32,
}

#[derive(Deserialize, Debug)]
pub struct InventoryPlaceQuery {
    pub item_name: Option<String>,
    pub place_type: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct InventoryItemQuery {
    pub place_name: Option<String>,
    pub place_type: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct User {
    pub user_id: i32,
    pub user_firstname: Option<String>,
    pub user_lastname: Option<String>,
    pub user_email: Option<String>,
    pub user_password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub user_email: Option<String>,
    pub user_password: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub token: String,
}

// the input to our `create_user` handler
#[derive(Deserialize, Debug)]
pub struct CreateUser {
    pub username: Option<String>,
    pub email: Option<String>,
}

// the output to our `create_user` handler
// #[derive(Serialize, Debug)]
// pub struct User {
//     pub id: u64,
//     pub username: String,
// }
