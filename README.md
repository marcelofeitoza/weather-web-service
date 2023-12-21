# Rust Weather Web Service

Implementation of a weather web service using Rust's web framework Axum, using [SQLX](https://github.com/launchbadge/sqlx), [serde](https://github.com/serde-rs/serde), [serde_json](https://github.com/serde-rs/json), along with [PostgreSQL](https://www.postgresql.org/) and [Redis](https://redis.io/), for database storage and caching system, respectively.

In order to store last cities called for checking weather Postgres was used, resulting in consulting times of around 1.7 seconds.

For improving request times, Redis was used, which resulted in consulting times that vary from 4 to 10 milliseconds.

## Running

For running the application you will need [Rust](https://www.rust-lang.org/tools/install) and [Docker](https://docs.docker.com/engine/install/) installed:

- Starting Postgres, Redis, and PostgresAdmin

  ```sh
  docker compose up -d # -d is for dettached mode- terminal will be freed as soon as everything goes well
  ```

- Running Rust web service

  ```sh
  cargo run
  ```

After running `cargo run` the application will try and connect to both Postgres and Redis (which might be already running with `docker compose up -d`) it will be available at [http://localhost:3000/](http://localhost:3000/).

For making a request for a city you will simply need to access [http://localhost:3000/weather/`your-city-name`](http://localhost:3000/weather/`your-city-name`) and replace `your-city-name` with the name of the city you want to fetch, which will result in something like this:

```json
{
    "city":"São Paulo",
    "forecasts":[
        {
            "date":"2023-12-21T00:00",
            "temperature":"19.9"
        },
        ...
    ]
}
```

In this example, this was the chosen city: [http://localhost:3000/weather/S%C3%A3o%20Paulo](http://localhost:3000/weather/S%C3%A3o%20Paulo).
