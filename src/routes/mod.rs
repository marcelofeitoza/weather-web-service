pub mod weather_routes;
use weather_routes::{weather};

use axum::{routing::get, response::Redirect};

use crate::models::weather_models::AppState;

pub fn weather_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/weather/:city", get(weather))
        .route("/", get(|| async {
            println!("Redirecting to /weather/Montana");
            Redirect::permanent("/weather/Montana")
        }))
}