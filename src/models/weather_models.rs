use std::time::Duration;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_client: redis::Client,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let db_connection_str = dotenv::var("DATABASE_URL")?;
        let db_pool = PgPool::connect(&db_connection_str).await?;

        let redis_url = dotenv::var("REDIS_URL")?;
        let redis_client = redis::Client::open(redis_url)?;

        Ok(AppState { db_pool, redis_client })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeoResponse {
    pub results: Vec<LatLong>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct LatLong {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize)]
pub struct WeatherParams {
    pub city: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub hourly: Hourly,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherDisplay {
    pub duration: String,
    pub city: String,
    pub forecasts: Vec<Forecast>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub date: String,
    pub temperature: String,
}

impl WeatherDisplay {
    pub fn new(city: &str, response: WeatherResponse, duration: Duration) -> Self {
        let display = WeatherDisplay {
            duration: format!("{:?}", duration),
            city: city.to_string(),
            forecasts: response
                .hourly
                .time
                .iter()
                .zip(response.hourly.temperature_2m.iter())
                .map(|(date, temperature)| Forecast {
                    date: date.to_string(),
                    temperature: temperature.to_string(),
                })
                .collect(),
        };

        display
    }
}
