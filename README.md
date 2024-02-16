# Rust Weather Web Service

Implementation of a weather web service using Rust's web framework Axum, using [SQLX](https://github.com/launchbadge/sqlx), [serde](https://github.com/serde-rs/serde), [serde_json](https://github.com/serde-rs/json), along with [PostgreSQL](https://www.postgresql.org/) and [Redis](https://redis.io/), for database storage and caching system, respectively.

In order to store last cities called for checking weather Postgres was used, resulting in consulting times of around 1.7 seconds.

For improving request times, Redis was used, which resulted in consulting times that vary from 4 to 10 milliseconds.

---

## Running

For running the application you will need [Rust](https://www.rust-lang.org/tools/install) and [Docker](https://docs.docker.com/engine/install/) installed:

- Starting Postgres, Redis, and PostgresAdmin

  ```sh
  docker compose up -d # -d is for dettached mode- terminal will be freed as soon as everything goes well
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

- Running the database, redis, and PostgresAdmin

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

For making a request for a city you will simply need to access [http://localhost:3000/weather/`your-city-name`](http://localhost:3000/weather/`your-city-name`) and replace `your-city-name` with the name of the city you want to fetch, which will result in something like this:

```json
{
  "duration": "893.557667ms",
  "city": "São Paulo",
  "forecasts": [
    {
      "date": "2024-02-16T00:00",
      "temperature": "19.3"
    },
    {
      "date": "2024-02-16T01:00",
      "temperature": "19.3"
    },
    {
      "date": "2024-02-16T02:00",
      "temperature": "19.3"
    },
    {
      "date": "2024-02-16T03:00",
      "temperature": "19.3"
    },
    {
      "date": "2024-02-16T04:00",
      "temperature": "19.3"
    },
    {
      "date": "2024-02-16T05:00",
      "temperature": "19.3"
    },
    {
      "date": "2024-02-16T06:00",
      "temperature": "19.3"
    },
    {
      "date": "2024-02-16T07:00",
      "temperature": "19.2"
    },
    {
      "date": "2024-02-16T08:00",
      "temperature": "19.2"
    },
    {
      "date": "2024-02-16T09:00",
      "temperature": "19.1"
    },
    {
      "date": "2024-02-16T10:00",
      "temperature": "19.2"
    },
    {
      "date": "2024-02-16T11:00",
      "temperature": "19.7"
    },
    {
      "date": "2024-02-16T12:00",
      "temperature": "20.6"
    },
    {
      "date": "2024-02-16T13:00",
      "temperature": "22.3"
    },
    {
      "date": "2024-02-16T14:00",
      "temperature": "23.7"
    },
    {
      "date": "2024-02-16T15:00",
      "temperature": "24.6"
    },
    {
      "date": "2024-02-16T16:00",
      "temperature": "25"
    },
    {
      "date": "2024-02-16T17:00",
      "temperature": "24.8"
    },
    {
      "date": "2024-02-16T18:00",
      "temperature": "23.8"
    },
    {
      "date": "2024-02-16T19:00",
      "temperature": "22.9"
    },
    {
      "date": "2024-02-16T20:00",
      "temperature": "22.5"
    },
    {
      "date": "2024-02-16T21:00",
      "temperature": "22"
    },
    {
      "date": "2024-02-16T22:00",
      "temperature": "21.3"
    },
    {
      "date": "2024-02-16T23:00",
      "temperature": "20.6"
    },
    {
      "date": "2024-02-17T00:00",
      "temperature": "20"
    },
    {
      "date": "2024-02-17T01:00",
      "temperature": "19.5"
    },
    {
      "date": "2024-02-17T02:00",
      "temperature": "19"
    },
    {
      "date": "2024-02-17T03:00",
      "temperature": "18.5"
    },
    {
      "date": "2024-02-17T04:00",
      "temperature": "18.1"
    },
    {
      "date": "2024-02-17T05:00",
      "temperature": "17.6"
    },
    {
      "date": "2024-02-17T06:00",
      "temperature": "17.2"
    },
    {
      "date": "2024-02-17T07:00",
      "temperature": "16.7"
    },
    {
      "date": "2024-02-17T08:00",
      "temperature": "16.3"
    },
    {
      "date": "2024-02-17T09:00",
      "temperature": "16.2"
    },
    {
      "date": "2024-02-17T10:00",
      "temperature": "16.6"
    },
    {
      "date": "2024-02-17T11:00",
      "temperature": "18.5"
    },
    {
      "date": "2024-02-17T12:00",
      "temperature": "21"
    },
    {
      "date": "2024-02-17T13:00",
      "temperature": "23.3"
    },
    {
      "date": "2024-02-17T14:00",
      "temperature": "25.3"
    },
    {
      "date": "2024-02-17T15:00",
      "temperature": "26.7"
    },
    {
      "date": "2024-02-17T16:00",
      "temperature": "27.6"
    },
    {
      "date": "2024-02-17T17:00",
      "temperature": "28"
    },
    {
      "date": "2024-02-17T18:00",
      "temperature": "27.4"
    },
    {
      "date": "2024-02-17T19:00",
      "temperature": "24.5"
    },
    {
      "date": "2024-02-17T20:00",
      "temperature": "24.1"
    },
    {
      "date": "2024-02-17T21:00",
      "temperature": "23.3"
    },
    {
      "date": "2024-02-17T22:00",
      "temperature": "22.1"
    },
    {
      "date": "2024-02-17T23:00",
      "temperature": "21.3"
    },
    {
      "date": "2024-02-18T00:00",
      "temperature": "20.7"
    },
    {
      "date": "2024-02-18T01:00",
      "temperature": "20.2"
    },
    {
      "date": "2024-02-18T02:00",
      "temperature": "19.8"
    },
    {
      "date": "2024-02-18T03:00",
      "temperature": "19.5"
    },
    {
      "date": "2024-02-18T04:00",
      "temperature": "19.2"
    },
    {
      "date": "2024-02-18T05:00",
      "temperature": "18.9"
    },
    {
      "date": "2024-02-18T06:00",
      "temperature": "18.5"
    },
    {
      "date": "2024-02-18T07:00",
      "temperature": "18.4"
    },
    {
      "date": "2024-02-18T08:00",
      "temperature": "18.5"
    },
    {
      "date": "2024-02-18T09:00",
      "temperature": "18.4"
    },
    {
      "date": "2024-02-18T10:00",
      "temperature": "19"
    },
    {
      "date": "2024-02-18T11:00",
      "temperature": "20.6"
    },
    {
      "date": "2024-02-18T12:00",
      "temperature": "22.5"
    },
    {
      "date": "2024-02-18T13:00",
      "temperature": "24.4"
    },
    {
      "date": "2024-02-18T14:00",
      "temperature": "25.8"
    },
    {
      "date": "2024-02-18T15:00",
      "temperature": "26.9"
    },
    {
      "date": "2024-02-18T16:00",
      "temperature": "26.9"
    },
    {
      "date": "2024-02-18T17:00",
      "temperature": "26.3"
    },
    {
      "date": "2024-02-18T18:00",
      "temperature": "25.5"
    },
    {
      "date": "2024-02-18T19:00",
      "temperature": "24.9"
    },
    {
      "date": "2024-02-18T20:00",
      "temperature": "24.5"
    },
    {
      "date": "2024-02-18T21:00",
      "temperature": "23.8"
    },
    {
      "date": "2024-02-18T22:00",
      "temperature": "22.8"
    },
    {
      "date": "2024-02-18T23:00",
      "temperature": "22"
    },
    {
      "date": "2024-02-19T00:00",
      "temperature": "21.4"
    },
    {
      "date": "2024-02-19T01:00",
      "temperature": "20.8"
    },
    {
      "date": "2024-02-19T02:00",
      "temperature": "20.3"
    },
    {
      "date": "2024-02-19T03:00",
      "temperature": "19.9"
    },
    {
      "date": "2024-02-19T04:00",
      "temperature": "19.5"
    },
    {
      "date": "2024-02-19T05:00",
      "temperature": "19.3"
    },
    {
      "date": "2024-02-19T06:00",
      "temperature": "18.9"
    },
    {
      "date": "2024-02-19T07:00",
      "temperature": "18.4"
    },
    {
      "date": "2024-02-19T08:00",
      "temperature": "18.1"
    },
    {
      "date": "2024-02-19T09:00",
      "temperature": "17.7"
    },
    {
      "date": "2024-02-19T10:00",
      "temperature": "18"
    },
    {
      "date": "2024-02-19T11:00",
      "temperature": "19.9"
    },
    {
      "date": "2024-02-19T12:00",
      "temperature": "22.7"
    },
    {
      "date": "2024-02-19T13:00",
      "temperature": "24.4"
    },
    {
      "date": "2024-02-19T14:00",
      "temperature": "26.1"
    },
    {
      "date": "2024-02-19T15:00",
      "temperature": "27.3"
    },
    {
      "date": "2024-02-19T16:00",
      "temperature": "28"
    },
    {
      "date": "2024-02-19T17:00",
      "temperature": "28.3"
    },
    {
      "date": "2024-02-19T18:00",
      "temperature": "28.2"
    },
    {
      "date": "2024-02-19T19:00",
      "temperature": "27.4"
    },
    {
      "date": "2024-02-19T20:00",
      "temperature": "26.2"
    },
    {
      "date": "2024-02-19T21:00",
      "temperature": "25"
    },
    {
      "date": "2024-02-19T22:00",
      "temperature": "23.8"
    },
    {
      "date": "2024-02-19T23:00",
      "temperature": "22.6"
    },
    {
      "date": "2024-02-20T00:00",
      "temperature": "21.7"
    },
    {
      "date": "2024-02-20T01:00",
      "temperature": "21.3"
    },
    {
      "date": "2024-02-20T02:00",
      "temperature": "21.1"
    },
    {
      "date": "2024-02-20T03:00",
      "temperature": "20.9"
    },
    {
      "date": "2024-02-20T04:00",
      "temperature": "20.5"
    },
    {
      "date": "2024-02-20T05:00",
      "temperature": "20.1"
    },
    {
      "date": "2024-02-20T06:00",
      "temperature": "19.6"
    },
    {
      "date": "2024-02-20T07:00",
      "temperature": "19"
    },
    {
      "date": "2024-02-20T08:00",
      "temperature": "18.3"
    },
    {
      "date": "2024-02-20T09:00",
      "temperature": "18.3"
    },
    {
      "date": "2024-02-20T10:00",
      "temperature": "19.3"
    },
    {
      "date": "2024-02-20T11:00",
      "temperature": "20.8"
    },
    {
      "date": "2024-02-20T12:00",
      "temperature": "22.3"
    },
    {
      "date": "2024-02-20T13:00",
      "temperature": "23.6"
    },
    {
      "date": "2024-02-20T14:00",
      "temperature": "24.7"
    },
    {
      "date": "2024-02-20T15:00",
      "temperature": "25.6"
    },
    {
      "date": "2024-02-20T16:00",
      "temperature": "26.4"
    },
    {
      "date": "2024-02-20T17:00",
      "temperature": "26.9"
    },
    {
      "date": "2024-02-20T18:00",
      "temperature": "26.7"
    },
    {
      "date": "2024-02-20T19:00",
      "temperature": "25.5"
    },
    {
      "date": "2024-02-20T20:00",
      "temperature": "23.6"
    },
    {
      "date": "2024-02-20T21:00",
      "temperature": "22.2"
    },
    {
      "date": "2024-02-20T22:00",
      "temperature": "21.6"
    },
    {
      "date": "2024-02-20T23:00",
      "temperature": "21.3"
    },
    {
      "date": "2024-02-21T00:00",
      "temperature": "21.2"
    },
    {
      "date": "2024-02-21T01:00",
      "temperature": "20.9"
    },
    {
      "date": "2024-02-21T02:00",
      "temperature": "20.7"
    },
    {
      "date": "2024-02-21T03:00",
      "temperature": "20.6"
    },
    {
      "date": "2024-02-21T04:00",
      "temperature": "20.5"
    },
    {
      "date": "2024-02-21T05:00",
      "temperature": "20.4"
    },
    {
      "date": "2024-02-21T06:00",
      "temperature": "20.4"
    },
    {
      "date": "2024-02-21T07:00",
      "temperature": "19.2"
    },
    {
      "date": "2024-02-21T08:00",
      "temperature": "19"
    },
    {
      "date": "2024-02-21T09:00",
      "temperature": "19.2"
    },
    {
      "date": "2024-02-21T10:00",
      "temperature": "20.2"
    },
    {
      "date": "2024-02-21T11:00",
      "temperature": "21.6"
    },
    {
      "date": "2024-02-21T12:00",
      "temperature": "23"
    },
    {
      "date": "2024-02-21T13:00",
      "temperature": "24.5"
    },
    {
      "date": "2024-02-21T14:00",
      "temperature": "26"
    },
    {
      "date": "2024-02-21T15:00",
      "temperature": "27.2"
    },
    {
      "date": "2024-02-21T16:00",
      "temperature": "28"
    },
    {
      "date": "2024-02-21T17:00",
      "temperature": "28.5"
    },
    {
      "date": "2024-02-21T18:00",
      "temperature": "28.4"
    },
    {
      "date": "2024-02-21T19:00",
      "temperature": "27.5"
    },
    {
      "date": "2024-02-21T20:00",
      "temperature": "26"
    },
    {
      "date": "2024-02-21T21:00",
      "temperature": "24.6"
    },
    {
      "date": "2024-02-21T22:00",
      "temperature": "23.3"
    },
    {
      "date": "2024-02-21T23:00",
      "temperature": "22.1"
    },
    {
      "date": "2024-02-22T00:00",
      "temperature": "21.2"
    },
    {
      "date": "2024-02-22T01:00",
      "temperature": "20.7"
    },
    {
      "date": "2024-02-22T02:00",
      "temperature": "20.6"
    },
    {
      "date": "2024-02-22T03:00",
      "temperature": "20.4"
    },
    {
      "date": "2024-02-22T04:00",
      "temperature": "20"
    },
    {
      "date": "2024-02-22T05:00",
      "temperature": "19.6"
    },
    {
      "date": "2024-02-22T06:00",
      "temperature": "19.3"
    },
    {
      "date": "2024-02-22T07:00",
      "temperature": "18.8"
    },
    {
      "date": "2024-02-22T08:00",
      "temperature": "18.4"
    },
    {
      "date": "2024-02-22T09:00",
      "temperature": "18.8"
    },
    {
      "date": "2024-02-22T10:00",
      "temperature": "20.4"
    },
    {
      "date": "2024-02-22T11:00",
      "temperature": "22.8"
    },
    {
      "date": "2024-02-22T12:00",
      "temperature": "24.9"
    },
    {
      "date": "2024-02-22T13:00",
      "temperature": "26.2"
    },
    {
      "date": "2024-02-22T14:00",
      "temperature": "27.1"
    },
    {
      "date": "2024-02-22T15:00",
      "temperature": "27.9"
    },
    {
      "date": "2024-02-22T16:00",
      "temperature": "28.5"
    },
    {
      "date": "2024-02-22T17:00",
      "temperature": "29"
    },
    {
      "date": "2024-02-22T18:00",
      "temperature": "29.1"
    },
    {
      "date": "2024-02-22T19:00",
      "temperature": "28.8"
    },
    {
      "date": "2024-02-22T20:00",
      "temperature": "28"
    },
    {
      "date": "2024-02-22T21:00",
      "temperature": "27.2"
    },
    {
      "date": "2024-02-22T22:00",
      "temperature": "26.4"
    },
    {
      "date": "2024-02-22T23:00",
      "temperature": "25.5"
    }
  ]
}
```

In this example, this was the chosen city: [http://localhost:3000/weather/S%C3%A3o%20Paulo](http://localhost:3000/weather/S%C3%A3o%20Paulo).
