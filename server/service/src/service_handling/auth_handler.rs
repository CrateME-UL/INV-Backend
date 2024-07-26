use crate::{create_jwt, error::Error::WrongCredentialsError, verify_hash, WebResult};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use domain::{LoginRequest, LoginResponse};
use repository::get_user_db;
use sqlx::PgPool;

pub async fn login_handler(
    body: LoginRequest,
    Extension(pool): Extension<PgPool>,
) -> WebResult<impl IntoResponse> {
    println!("Login Request");

    let user_email = body.user_email.as_ref().ok_or(WrongCredentialsError)?;
    let user_password = body.user_password.as_ref().ok_or(WrongCredentialsError)?;

    let user = get_user_db(user_email.to_string(), Extension(pool.clone()))
        .await
        .map_err(|_| WrongCredentialsError)?;

    let is_password_valid = verify_hash(
        user_password,
        user.user_password.as_deref().ok_or(WrongCredentialsError)?,
    );

    if user.user_email == Some(user_email.clone()) && is_password_valid {
        let token = create_jwt(&user.user_id).map_err(|_| WrongCredentialsError)?;
        let response = LoginResponse { token };
        Ok((StatusCode::OK, Json(response)).into_response())
    } else {
        Err(WrongCredentialsError.into())
    }
}
