use chrono::NaiveDateTime;
use tokio_postgres::{Client, NoTls};
use crate::adsb::model::ADSBMessage;


/* Initialize the PostgreSQL connection. */

pub async fn init_db(conn_str: &str) -> Result<Client, Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });
    Ok(client)
}

/* Insert processed ADS‑B data into the database.*/

pub async fn store_aircraft_data(
    client: &Client,
    message: &ADSBMessage,
    rate_of_climb: f64,
    horizontal_speed: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert DateTime<Utc> to NaiveDateTime, then format it as a string.
    let naive_dt: NaiveDateTime = message.received_at.naive_utc();
    let ts_str = naive_dt.format("%Y-%m-%d %H:%M:%S").to_string();

    // Use PostgreSQL's to_timestamp function to convert the string parameter.
    client.execute(
        "INSERT INTO aircraft_logs
            (icao_address, altitude, latitude, longitude, rate_of_climb, horizontal_speed, received_at)
         VALUES ($1, $2, $3, $4, $5, $6, to_timestamp($7, 'YYYY-MM-DD HH24:MI:SS'))",
        &[
            &message.raw.icao_address,
            &message.raw.altitude,
            &message.raw.latitude,
            &message.raw.longitude,
            &rate_of_climb,
            &horizontal_speed,
            &ts_str,
        ],
    ).await?;
    
    println!(
        "Logged aircraft {}: altitude {}, rate of climb {:.2} ft/s, horizontal speed {:.2} m/s \n",
        message.raw.icao_address,
        message.raw.altitude,
        rate_of_climb,
        horizontal_speed
    );
    Ok(())
}
