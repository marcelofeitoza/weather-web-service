CREATE TABLE IF NOT EXISTS cities (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    lat FLOAT8,
    long FLOAT8
);

CREATE INDEX IF NOT EXISTS cities_name_idx ON cities (name);
