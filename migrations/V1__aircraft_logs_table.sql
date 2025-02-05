CREATE TABLE aircraft_logs (
    id SERIAL PRIMARY KEY,
    icao_address INTEGER,
    altitude INTEGER,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    rate_of_climb DOUBLE PRECISION,
    horizontal_speed DOUBLE PRECISION,
    received_at TIMESTAMP WITHOUT TIME ZONE
);
