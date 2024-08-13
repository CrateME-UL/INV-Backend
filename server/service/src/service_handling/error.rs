use domain::ErrorResponse;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("jwt token not valid")]
    JWTToken,
    #[error("jwt token creation error")]
    JWTTokenCreation,
    #[error("no auth header")]
    NoAuthHeader,
    #[error("invalid auth header")]
    InvalidAuthHeader,
    #[error("no permission")]
    NoPermission,
    #[error("user already exists")]
    UserAlreadyExists,
}

impl warp::reject::Reject for AuthError {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(e) = err.find::<AuthError>() {
        match e {
            AuthError::WrongCredentials => (StatusCode::FORBIDDEN, e.to_string()),
            AuthError::NoPermission => (StatusCode::UNAUTHORIZED, e.to_string()),
            AuthError::JWTToken => (StatusCode::UNAUTHORIZED, e.to_string()),
            AuthError::JWTTokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            _ => (StatusCode::BAD_REQUEST, e.to_string()),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    let json = warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}
