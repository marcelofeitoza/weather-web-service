use std::str::from_utf8;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use base64::Engine;

pub mod weather_models;

pub struct AppError {
    error: anyhow::Error,
    status_code: Option<StatusCode>,
}

impl AppError {
    pub fn new<E>(err: E, status_code: Option<StatusCode>) -> Self
    where
        E: Into<anyhow::Error>,
    {
        Self {
            error: err.into(),
            status_code,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.status_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, format!("Something went wrong: {}", self.error)).into_response()
    }
}

impl<E> From<E> for AppError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::new(err, None)
    }
}


pub struct User;

#[async_trait]
impl<S> FromRequestParts<S> for User
    where
        S: Send + Sync,
{
    type Rejection = axum::http::Response<axum::body::Body>;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Basic ") {
                let credentials = auth_header.trim_start_matches("Basic ");
                let decoded = base64::engine::general_purpose::STANDARD
                    .decode(credentials.as_bytes())
                    .unwrap_or_default();

                let credential_str = from_utf8(&decoded).unwrap_or("");

                if credential_str == "forecast:forecast" {
                    return Ok(User);
                }
            }
        }

        let reject_response = axum::http::Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(
                "WWW-Authenticate",
                "Basic realm=\"Please enter your credentials\"",
            )
            .body(axum::body::Body::from("Unauthorized"))
            .unwrap();

        Err(reject_response)
    }
}
