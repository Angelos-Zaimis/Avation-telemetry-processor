use tokio::net::UdpSocket;
use tokio_postgres::{Client, NoTls};
use std::error::Error;
use nom::{
    bits::{bits, complete::take},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct ADSBMessage {
    pub downlink_format: u8,
    pub capability: u8,
    pub icao_address: u32,
    pub altitude: u16,
    pub latitude: f64,
    pub longitude: f64,
}

fn parse_adsb_message(input: &[u8]) -> IResult<&[u8], ADSBMessage> {
    bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(|input| {
        let (input, downlink_format): (_, u8) = take(5u8)(input)?;
        let (input, capability): (_, u8) = take(3u8)(input)?;
        let (input, icao_address): (_, u32) = take(24u32)(input)?;
        let (input, altitude_raw): (_, u16) = take(12u16)(input)?;
        let (input, latitude_raw): (_, u32) = take(17u32)(input)?;
        let (input, longitude_raw): (_, u32) = take(17u32)(input)?;

        let altitude = altitude_raw * 25;
        let latitude = (latitude_raw as f64) * (180.0 / (1 << 17)) - 90.0;
        let longitude = (longitude_raw as f64) * (360.0 / (1 << 17)) - 180.0;

        Ok((input, ADSBMessage {
            downlink_format,
            capability,
            icao_address,
            altitude_raw,
            latitude_raw,
            longitude_raw
        }))
    })(input)
}

fn process_data(data: &[u8]) {
    // Parse the data
    let result = parse_adsb_message(data);

    let (client, connection) = tokio_postgres::connect("host=localhost dbname=postgres port=5432", NoTls);

    match result {
        Ok((_, message)) => {
            store_adsb_data() },
        Err(e) => {
            eprintln!("Failed to parse ADS-B message: {:?}", e);
        }
    }

    Ok(())
}


fn store_adsb_data(client: &Client, message: &ADSBMessage) {

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
            VALUES (&1, &2, &3, &4. 5&, &6)
        ",
        &[
            &message.downlink_format,
            &message.capability,
            &message.icao_address,
            &message.altitude,
            &message.latitude,
            &message.longitude
        ],
    );

    println!("Date inserted successufly");
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Bind the socket to a specific address and port

    let socket = UdpSocket::bind("0.0.0.0:3000").await?;
    println!("Listening for telemetry data on UDP port 3000...");

    let mut buf = vec![0; 1024]; // Buffer to store incoming data

    loop {
        // Wait for an incoming packet
        let (len, addr) = socket.recv_from(&mut buf).await?;
        // Process the received data
        process_data(&buf[..len]);
    }
}