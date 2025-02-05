pub mod model;
pub mod parser;

use chrono::Utc;
use crate::db;
use crate::state::AircraftStateManager;
use crate::adsb::model::ADSBMessage;

/* Process a UDP data packet containing ADS‑B telemetry.*/

pub async fn process_data(
    data: &[u8],
    state_manager: std::sync::Arc<AircraftStateManager>,
    db_client: std::sync::Arc<tokio_postgres::Client>,
) -> Result<(), Box<dyn std::error::Error>> {
    
    match parser::parse_raw_adsb_message(data) {
        Ok((_, raw_msg)) => {
            let now = Utc::now();
            let message = ADSBMessage {
                raw: raw_msg,
                received_at: now,
            };

            let (rate_of_climb, horizontal_speed) = state_manager.compute_metrics(&message).await;
            db::store_aircraft_data(&db_client, &message, rate_of_climb, horizontal_speed).await?;
        }
        Err(e) => {
            eprintln!("Failed to parse ADS‑B message: {:?}", e);
        }
    }
    Ok(())
}