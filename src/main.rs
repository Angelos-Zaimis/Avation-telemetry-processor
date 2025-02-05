mod adsb;
mod db;
mod state;
mod utils;

use tokio::net::UdpSocket;
use std::sync::Arc;
use state::AircraftStateManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* Initialize the PostgreSQL database connection. */
    let db_client = db::init_db("host=localhost port=5434 dbname=postgres user=postgres").await?;
    let db_client = Arc::new(db_client);

    /* Create the shared state manager.*/
    let state_manager = Arc::new(AircraftStateManager::new());

    /* Bind the UDP socket.*/
    let socket = UdpSocket::bind("0.0.0.0:3000").await?;
    println!("Listening for ADS‑B telemetry on UDP port 3000...");

    let mut buf = vec![0u8; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes from {}", len, addr);

        let data = buf[..len].to_vec();
        let state_manager = state_manager.clone();
        let db_client = db_client.clone();

        tokio::spawn(async move {
            if let Err(e) = adsb::process_data(&data, state_manager, db_client).await {
                eprintln!("Error processing data: {:?}", e);
            }
        });
    }
}