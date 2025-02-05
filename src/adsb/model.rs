use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct RawADSBMessage {
    pub downlink_format: i16,
    pub capability: i16,
    pub icao_address: i32,
    pub altitude: i32,   // altitude in feet
    pub latitude: f64,   // in degrees
    pub longitude: f64,  // in degrees
}

#[derive(Debug, Clone)]
pub struct ADSBMessage {
    pub raw: RawADSBMessage,
    pub received_at: DateTime<Utc>,
}