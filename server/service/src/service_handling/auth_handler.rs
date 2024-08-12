use std::error::Error;

use crate::{create_jwt, error::Error::WrongCredentialsError, verify_hash};
use axum::Json;
use domain::{LoginRequest, LoginResponse};
use repository::get_user_db;
use serde::Serialize;
use serde_json::Value;

fn map_to_value<T>(items: T) -> Value
where
    T: Serialize,
{
    serde_json::to_value(items).unwrap()
}

async fn handle_db_result<T, F>(db_call: F) -> Result<Value, Box<dyn Error>>
where
    F: std::future::Future<Output = Result<T, Box<dyn Error>>>,
    T: Serialize,
{
    match db_call.await {
        Ok(result) => Ok(map_to_value(result)),
        Err(err) => {
            eprintln!("Error: {}", err);
            Err(err)
        }
    }
}

pub async fn login_service(
    payload: Json<LoginRequest>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let user = get_user_db(&payload)
        .await
        .map_err(|_| Box::new(WrongCredentialsError) as Box<dyn Error>)?;

    let is_password_valid = verify_hash(
        &payload.user_password.as_deref().unwrap_or_default(),
        &user.user_password.as_deref().unwrap_or_default(),
    );

    let response_user_db = async {
        if user.user_email == payload.user_email && is_password_valid {
            create_jwt(&user.user_id)
                .map(|token| LoginResponse { token })
                .map_err(|_| Box::new(WrongCredentialsError) as Box<dyn Error>)
        } else {
            Err(Box::new(WrongCredentialsError) as Box<dyn Error>)
        }
    };

    handle_db_result(response_user_db).await
}
