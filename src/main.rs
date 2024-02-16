mod models;
mod routes;
mod services;

use axum::routing::get;
use models::weather_models::AppState;
use routes::weather_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_state = AppState::new().await?;

    let app = axum::Router::new()
        .nest("/", weather_router())
        .route("/health", get(|| async { "Hello, weather!" }))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Listening at http://0.0.0.0:3000/ 🦀");

    axum::serve(listener, app).await.unwrap();

    Ok(())
}