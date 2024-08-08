use std::error::Error;

use crate::{create_jwt, error::Error::WrongCredentialsError, verify_hash, WebResult};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use domain::{LoginRequest, LoginResponse};
use repository::get_user_db;
use serde::Serialize;
use serde_json::Value;
use warp::reply::Reply;
// use repository::get_user_db;
//use sqlx::PgPool;

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

// fn map_token(result: Result<LoginResponse, Box<dyn Error>>) -> Result<Value, Box<dyn Error>> {
//     println!("Login Request");
//     match result {
//         Ok(response) => Ok(map_to_value(response)),
//         Err(err) => {
//             eprintln!("Error: {}", err);
//             Err(err)
//         }
//     }
// }

// pub async fn login_handler(
//     Json(body): Json<LoginRequest>,
//     //Extension(pool): Extension<PgPool>,
// ) -> WebResult<impl IntoResponse> {
//     println!("Login Request");

//     let user_email = body.user_email.as_ref().ok_or(WrongCredentialsError)?;
//     let user_password = body.user_password.as_ref().ok_or(WrongCredentialsError)?;

//     // Create a Query<LoginRequest> to pass to get_user_db
//     let query = Query(LoginRequest {
//         user_email: Some(user_email.clone()),
//         user_password: Some(user_password.clone()),
//     });

//     let user = get_user_db(&query)
//         .await
//         .map_err(|_| WrongCredentialsError)?;

//     let is_password_valid = verify_hash(
//         user_password,
//         user.user_password.as_deref().ok_or(WrongCredentialsError)?,
//     );

//     if user.user_email == Some(user_email.clone()) && is_password_valid {
//         let token = create_jwt(&user.user_id).map_err(|_| WrongCredentialsError)?;
//         let response = LoginResponse { token };
//         Ok((StatusCode::OK, Json(response)).into_response())
//     } else {
//         Err(WrongCredentialsError.into())
//     }
// }
