pub mod weather_routes;

use weather_routes::{weather};

use crate::models::weather_models::AppState;

use axum::routing::get;

pub fn weather_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/weather/:city", get(weather))
}