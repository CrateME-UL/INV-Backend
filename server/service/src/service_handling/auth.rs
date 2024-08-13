use crate::{error::AuthError, Result, WebResult};
use chrono::prelude::*;
use domain::Claims;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue},
    reject, Filter, Rejection,
};

const AUTH_HEADER: &str = "x-authentication-token";
const JWT_SECRET: &[u8] = b"testsecret";

pub fn with_auth() -> impl Filter<Extract = (i32,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| (headers))
        .and_then(authorize)
}

pub fn create_jwt(uid: &i32) -> Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(300))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: *uid,
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| AuthError::JWTTokenCreation)
}

async fn authorize(headers: HeaderMap<HeaderValue>) -> WebResult<i32> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| reject::custom(AuthError::JWTToken))?;

            Ok(decoded.claims.sub)
        }
        Err(e) => Err(reject::custom(e)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(AUTH_HEADER) {
        Some(v) => v,
        None => return Err(AuthError::NoAuthHeader),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(AuthError::NoAuthHeader),
    };

    Ok(auth_header.to_owned())
}
