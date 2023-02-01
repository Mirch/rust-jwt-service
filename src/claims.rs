use std::{fmt::Display, future::Future, pin::Pin};

use actix_web::{Error, FromRequest, HttpResponse, ResponseError};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};

use crate::jwt::validate_token;

#[derive(Debug)]
struct TokenValidationError;

impl ResponseError for TokenValidationError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::Unauthorized().finish()
    }
}

impl Display for TokenValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token validation error.")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: i64,
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let extractor = BearerAuth::extract(req);

        Box::pin(async {
            let credentials = extractor.await.map_err(|_| TokenValidationError {})?;
            let token = credentials.token();
            let claims = validate_token(token.to_string()).map_err(|_| TokenValidationError {})?;

            Ok(claims)
        })
    }

    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        Self::from_request(req, &mut actix_web::dev::Payload::None)
    }
}
