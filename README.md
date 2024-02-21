# Rust Weather Web Service

Implementation of a weather web service using Rust's web framework Axum, using [SQLX](https://github.com/launchbadge/sqlx), [serde](https://github.com/serde-rs/serde), [serde_json](https://github.com/serde-rs/json), along with [PostgreSQL](https://www.postgresql.org/) and [Redis](https://redis.io/), for database storage and caching system, respectively.

In order to store last cities called for checking weather Postgres was used, resulting in consulting times of around 1.7 seconds.

For improving request times, Redis was used, which resulted in consulting times that vary from 4 to 10 milliseconds.

---

## Running

For running the application you will need [Rust](https://www.rust-lang.org/tools/install) and [Docker](https://docs.docker.com/engine/install/) installed:

- Starting Postgres and Redis

  ```sh
  docker compose up -d # -d is for dettached mode- terminal will be free
  ```

After running the application will connect to Postgres and Redis and will be ready to receive requests at [http://localhost:3000](http://localhost:3000).

---

## Development

- Comment out this part of the `docker-compose.yml` file:
  ```md
    #  Comment for local development
    weather-web-service:
      build: ./
      ports:
        - "3000:3000"
      depends_on:
        - postgres
        - redis
      environment:
        DATABASE_URL: "postgres://forecast:forecast@forecast_postgres:5432/forecast?sslmode=disable"
        REDIS_URL: "redis://redis/"
    # Comment for local development
  ```

- Running the database and redis

  ```sh
  docker compose up -d
  ```

- Running the application

  ```sh
  cargo run
  ```

---

## Testing

- Running the tests

  ```sh
  cd tests/
  ```

- Installing dependencies

  ```sh
  pnpm install # or npm install, or yarn
  ```

- Running the tests

  ```sh
  pnpm test # or npm test, or yarn test
  ```

---

For making a request for a city you will simply need to access [http://localhost:3000/weather/your-city-name](http://localhost:3000/weather/your-city-name) and replace `your-city-name` with the name of the city you want to fetch, which will result in something like this:

```json
{
  "duration": "893.557667ms",
  "city": "São Paulo",
  "forecasts": [
    {
      "date": "2024-02-16T00:00",
      "temperature": "19.3"
    },
    ...
  ]
}
```

In this example, this was the chosen city: [http://localhost:3000/weather/S%C3%A3o%20Paulo](http://localhost:3000/weather/S%C3%A3o%20Paulo).
