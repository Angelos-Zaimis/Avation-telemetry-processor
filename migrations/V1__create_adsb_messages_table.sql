CREATE TABLE IF NOT EXISTS adsb_messages (
                                             downlink_format SMALLINT,
                                             capability SMALLINT,
                                             icao_address INTEGER,
                                             altitude INTEGER,
                                             latitude DOUBLE PRECISION,
                                             longitude DOUBLE PRECISION
);
