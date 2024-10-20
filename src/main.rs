use tokio::net::UdpSocket;
use tokio_postgres::{NoTls};
use nom::{bits, IResult};
use nom::bits::complete::take;

#[derive(Debug, PartialEq)]
pub struct ADSBMessage {
    pub downlink_format: i16,
    pub capability: i16,
    pub icao_address: i32,
    pub altitude: i32,
    pub latitude: f64,
    pub longitude: f64,
}

fn parse_adsb_message(input: &[u8]) -> IResult<&[u8], ADSBMessage> {
    bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(|input| {
        let (input, downlink_format): (_, u8) = take(5u8)(input)?;
        let (input, capability): (_, u8) = take(3u8)(input)?;
        let (input, icao_address): (_, i32) = take(24u32)(input)?;
        let (input, altitude_raw): (_, u16) = take(12u16)(input)?;
        let (input, latitude_raw): (_, u32) = take(17u32)(input)?;
        let (input, longitude_raw): (_, u32) = take(17u32)(input)?;

        let altitude = altitude_raw as i32 * 25;
        let latitude = (latitude_raw as f64) * (180.0 / (1u64 << 17) as f64) - 90.0;
        let longitude = (longitude_raw as f64) * (360.0 / (1u64 << 17) as f64) - 180.0;

        Ok((input, ADSBMessage {
            downlink_format: downlink_format as i16,
            capability: capability as i16,
            icao_address,
            altitude,
            latitude,
            longitude,
        }))
    })(input)
}

async fn process_data(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the data
    let result = parse_adsb_message(data);

    // Create a connection to the database
    let (client, connection) = tokio_postgres::connect("host=localhost port=5434 dbname=postgres user=postgres", NoTls).await?;

    // Spawn a separate task for the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    match result {
        Ok((_, message)) => {
            store_adsb_data(&client, &message).await?;
        },
        Err(e) => {
            eprintln!("Failed to parse ADS-B message: {:?}", e);
        }
    }

    Ok(())
}

async fn store_adsb_data(client: &tokio_postgres::Client, message: &ADSBMessage) -> Result<(), Box<dyn std::error::Error>> {
    client.execute(
        "INSERT INTO adsb_messages
            (
                downlink_format,
                capability,
                icao_address,
                altitude,
                latitude,
                longitude
            )
            VALUES ($1, $2, $3, $4, $5, $6)",
        &[
            &message.downlink_format,
            &message.capability,
            &message.icao_address,
            &message.altitude,
            &message.latitude,
            &message.longitude
        ],
    ).await?;

    println!("Data inserted successfully");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the socket to a specific address and port
    let socket = UdpSocket::bind("0.0.0.0:3000").await?;
    println!("Listening for telemetry data on UDP port 3000...");

    let mut buf = vec![0; 1024]; // Buffer to store incoming data

    loop {
        // Wait for an incoming packet
        let (len, _addr) = socket.recv_from(&mut buf).await?;
        // Process the received data
        process_data(&buf[..len]).await?;
    }
}
