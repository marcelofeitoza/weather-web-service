use anyhow::Context;
use redis::Commands;

use crate::models::weather_models::{AppState, GeoResponse, LatLong, WeatherResponse};

pub struct WeatherService {}

impl WeatherService {
    pub fn new() -> Self {
        WeatherService {}
    }

    pub async fn fetch_lat_long(&self, city: &str) -> Result<LatLong, anyhow::Error> {
        let endpoint = format!(
    	"https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
    	city
	);
        let response = reqwest::get(&endpoint).await?.json::<GeoResponse>().await?;
        response.results.get(0).cloned().context("No results found")
    }

    pub async fn get_lat_long(
        &self,
        app_state: &AppState,
        name: &str,
    ) -> Result<LatLong, anyhow::Error> {
        let redis_client = &app_state.redis_client;
        let db_pool = &app_state.db_pool;

        let lat_long_redis_key = format!("lat_long:{}", name);

        // Reading from Redis
        let mut redis_conn: redis::Connection = redis_client.get_connection()?;
        let lat_long_redis: Option<String> = redis_conn.get(&lat_long_redis_key).unwrap();

        if let Some(lat_long_redis) = lat_long_redis {
            let lat_long: LatLong = serde_json::from_str(&lat_long_redis)?;
            return Ok(lat_long);
        }

        let lat_long = sqlx::query_as::<_, LatLong>(
            "SELECT lat AS latitude, long AS longitude FROM cities WHERE name = $1",
        )
        .bind(name)
        .fetch_optional(db_pool)
        .await?;

        let mut redis_conn: redis::Connection = redis_client.get_connection()?;

        let lat_long_json = serde_json::to_string(&lat_long)?;
        let _: () = redis_conn.set(&lat_long_redis_key, lat_long_json).unwrap();

        if let Some(lat_long) = lat_long {
            return Ok(lat_long);
        }

        let lat_long = self.fetch_lat_long(name).await?;
        sqlx::query("INSERT INTO cities (name, lat, long) VALUES ($1, $2, $3)")
            .bind(name)
            .bind(lat_long.latitude)
            .bind(lat_long.longitude)
            .execute(db_pool)
            .await?;

        // Writing to Redis because the data was not found in the redis cache and was not found in the database
        let mut redis_conn: redis::Connection = redis_client.get_connection()?;

        let lat_long_json = serde_json::to_string(&lat_long)?;
        let _: () = redis_conn.set(&lat_long_redis_key, lat_long_json).unwrap();

        Ok(lat_long)
    }

    pub async fn fetch_weather(
        &self,
        app_state: &AppState,
        lat_long: LatLong,
    ) -> Result<WeatherResponse, anyhow::Error> {
        let redis_client = &app_state.redis_client;

        let weather_redis_key = format!(
            "lat:{}-long:{}-weather-redis",
            lat_long.latitude, lat_long.longitude
        );

        let mut redis_conn: redis::Connection = redis_client.get_connection()?;
        let weather_redis: Option<String> = redis_conn.get(&weather_redis_key).unwrap();

        if let Some(weather_redis) = weather_redis {
            let weather: WeatherResponse = serde_json::from_str(&weather_redis)?;
            return Ok(weather);
        }

        let endpoint = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m",
            lat_long.latitude, lat_long.longitude
        );
        let response = reqwest::get(&endpoint)
            .await?
            .json::<WeatherResponse>()
            .await?;

        let mut redis_conn: redis::Connection = redis_client.get_connection()?;

        let _: () = redis_conn
            .set(&weather_redis_key, serde_json::to_string(&response)?)
            .unwrap();
        redis_conn.expire(&weather_redis_key, 15)?;

        Ok(response)
    }
}
