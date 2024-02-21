
use std::time::Instant;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use crate::{
    models::{
        AppError,
        weather_models::{
            AppState,
            WeatherDisplay,
            WeatherParams
        },
    },
    services::weather_service::{
        get_weather,
        get_lat_long
    }
};

pub async fn weather(
    Path(params): Path<WeatherParams>,
    State(app_state): State<AppState>,
) -> Result<String, AppError> {
    let city = &params.city;

    let start = Instant::now();

    let lat_long = get_lat_long(&app_state, &city).await.map_err(
        |err| AppError::new(err, Some(StatusCode::NOT_FOUND))
    )?;
    let weather = get_weather(&app_state, lat_long).await?;

    let duration = start.elapsed();

    let display = WeatherDisplay::new(&city, weather, duration);
    let json = serde_json::to_string(&display)?;

    println!("Weather request to {city} took: {:?}", duration);

    Ok(json)
}