use axum::{response::{IntoResponse, Response}, http::StatusCode};

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
