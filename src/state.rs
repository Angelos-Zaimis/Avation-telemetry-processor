use std::collections::HashMap;
use tokio::sync::Mutex;
use crate::adsb::model::ADSBMessage;

pub struct  AircraftStateManager {
    states: Mutex<HashMap<i32, ADSBMessage>>
}

impl AircraftStateManager {
    pub fn new() -> Self {
        Self {
            states: Mutex::new(HashMap::new()),
        }
    }
    
    /* Compute the rate of climb and horizontal speed, updating the stored state. */ 
    pub async fn compute_metrics(&self, new_message: &ADSBMessage) -> (f64, f64) {
        let mut rate_of_climb = 0.0;
        let mut horizontal_speed = 0.0;
        let mut states = self.states.lock().await;

    
        if let Some(old_message) = states.get(&new_message.raw.icao_address) {

            let dt = (new_message.received_at - old_message.received_at).num_seconds() as f64;
            if dt > 0.0 {
                rate_of_climb = (new_message.raw.altitude - old_message.raw.altitude) as f64 / dt;
                horizontal_speed = crate::utils::haversine_distance(
                    new_message.raw.latitude,
                    new_message.raw.longitude,
                    old_message.raw.latitude,
                    old_message.raw.longitude,
                ) / dt;
            }
        }

        /* Update the state with the latest message. */ 
        states.insert(new_message.raw.icao_address, new_message.clone());
        (rate_of_climb, horizontal_speed)
    }
    
}