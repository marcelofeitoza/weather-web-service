mod models;
mod services;

use crate::models::weather_models::{WeatherDisplay, WeatherParams};
use crate::services::weather_service::WeatherService;

use axum::async_trait;
use axum::extract::{FromRequestParts, Path, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use models::weather_models::AppState;
use std::str::from_utf8;
use std::time::Instant;

async fn index() -> &'static str {
    "Index"
}

struct User;

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
                let decoded = base64::decode(credentials).unwrap_or_default();
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

async fn stats(_user: User) -> &'static str {
    "We're authorized!"
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

async fn weather(
    Path(params): Path<WeatherParams>,
    State(app_state): State<AppState>,
) -> Result<String, AppError> {
    let start = Instant::now();

    let weather_service = WeatherService::new();
    let city = params.city.clone();

    let lat_long = weather_service
        .get_lat_long(&app_state, &params.city)
        .await?;
    let weather = weather_service.fetch_weather(&app_state, lat_long).await?;
    let display = WeatherDisplay::new(&city, weather);
    let json = serde_json::to_string(&display)?;

    let duration = start.elapsed();

    println!("Time elapsed for {}: {:?}", &city, duration);

    Ok(json)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_connection_str = "postgres://forecast:forecast@localhost:5432/forecast?sslmode=disable";
    let db_pool = sqlx::PgPool::connect(db_connection_str).await?;

    let redis_client = redis::Client::open("redis://127.0.0.1/")?;

    let app_state = AppState {
        db_pool: db_pool.clone(),
        redis_client: redis_client.clone(),
    };

    let app = axum::Router::new()
        .route("/", get(index))
        .route("/health", get(|| async { "Hello, weather!" }))
        .route("/weather/:city", get(weather))
        .route("/stats", get(stats))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening at http://127.0.0.1:3000/ 🦀");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
