use nom::{bits, IResult};
use nom::bits::complete::take;
use crate::adsb::model::RawADSBMessage;


/* Parse a raw ADS‑B message from a binary slice.*/

pub fn parse_raw_adsb_message(input: &[u8]) -> IResult<&[u8], RawADSBMessage> {
    bits::<_, _, nom::error::Error<(&[u8], usize)>, _, _>(|input| {
        
        let (input, downlink_format): (_, u8) = take(5u8)(input)?;
        let (input, capability): (_, u8) = take(3u8)(input)?;
        let (input, icao_address): (_, i32) = take(24u32)(input)?;
        let (input, altitude_raw): (_, u16) = take(12u16)(input)?;
        let (input, latitude_raw): (_, u32) = take(17u32)(input)?;
        let (input, longitude_raw): (_, u32) = take(17u32)(input)?;

        let altitude = altitude_raw as i32 * 25;
        let latitude = (latitude_raw as f64) * (180.0 / ((1u64 << 17) as f64)) - 90.0;
        let longitude = (longitude_raw as f64) * (360.0 / ((1u64 << 17) as f64)) - 180.0;

        Ok((input, RawADSBMessage {
            downlink_format: downlink_format as i16,
            capability: capability as i16,
            icao_address,
            altitude,
            latitude,
            longitude,
        }))
    })(input)
}