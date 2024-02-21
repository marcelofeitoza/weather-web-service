use anyhow::Context;
use redis::Commands;

use crate::models::weather_models::{AppState, GeoResponse, LatLong, Weather};

pub async fn fetch_lat_long(city: &str) -> Result<LatLong, anyhow::Error> {
    let endpoint = format!(
    "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
    city
);
    let response = reqwest::get(&endpoint).await?.json::<GeoResponse>().await?;
    response.results.get(0).cloned().context("No results found")
}

pub async fn get_lat_long(
    app_state: &AppState,
    name: &str,
) -> Result<LatLong, anyhow::Error> {
    let redis_client = &app_state.redis_client;
    let db_pool = &app_state.db_pool;

    let lat_long_redis_key = format!("lat_long:{}", name);

    let mut redis_conn: redis::Connection = redis_client.get_connection()?;
    let lat_long_redis: Option<String> = redis_conn.get(&lat_long_redis_key)?;

    if let Some(lat_long_redis) = lat_long_redis {
        let lat_long: LatLong = serde_json::from_str(&lat_long_redis)?;
        return Ok(lat_long);
    }

    let lat_long = sqlx::query_as::<_, LatLong>(
    "SELECT lat AS latitude, long AS longitude FROM cities WHERE name = $1"
    )
        .bind(name)
        .fetch_optional(db_pool)
        .await?;

    if let Some(lat_long) = lat_long {
        set_redis_value(&mut redis_conn, &lat_long_redis_key, &lat_long, 60)?;
        return Ok(lat_long);
    }

        let lat_long = fetch_lat_long(name).await?;
        sqlx::query("INSERT INTO cities (name, lat, long) VALUES ($1, $2, $3)")
            .bind(name)
            .bind(lat_long.latitude)
            .bind(lat_long.longitude)
            .execute(db_pool)
            .await?;

    set_redis_value(&mut redis_conn, &lat_long_redis_key, &lat_long, 60)?;

    Ok(lat_long)
}

pub async fn get_weather(
    app_state: &AppState,
    lat_long: LatLong,
) -> Result<Weather, anyhow::Error> {
    let redis_client = &app_state.redis_client;

    let weather_redis_key = format!(
        "weather:{}:{}",
        lat_long.latitude,
        lat_long.longitude
    );

    let mut redis_conn: redis::Connection = redis_client.get_connection()?;

    if let Ok(weather_redis) = redis_conn.get::<_, Option<String>>(&weather_redis_key) {
        if let Some(weather_redis) = weather_redis {
            let weather: Weather = serde_json::from_str(&weather_redis)?;
            return Ok(weather);
        }
    }

    let endpoint = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m",
        lat_long.latitude,
        lat_long.longitude
    );
    let response = reqwest::get(&endpoint)
        .await?
        .json::<Weather>()
        .await?;

    set_redis_value(&mut redis_conn, &weather_redis_key, &response, 15)?;

    Ok(response)
}

fn set_redis_value<T: serde::Serialize>(
    redis_conn: &mut redis::Connection,
    key: &str,
    value: &T,
    ttl: i64,
) -> Result<(), redis::RedisError> {
    let json_value = match serde_json::to_string(value) {
        Ok(v) => v,
        Err(_) => return Err(redis::RedisError::from((redis::ErrorKind::TypeError, "Failed to serialize value"))),
    };
    redis_conn.set(key, &json_value)?;
    redis_conn.expire(key, ttl)?;
    Ok(())
}