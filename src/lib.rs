use std::convert::TryInto;
use std::result::Result;
use std::error::Error;

/**
 * return (soc, f32)
 */
pub fn parse_90(buffer: [u8; 13]) -> Result<(f32, f32), Box<dyn Error>> {
    let current = u16::from_be_bytes(buffer[8..10].try_into()?);
    let soc = u16::from_be_bytes(buffer[10..12].try_into()?);

    Ok((soc as f32 / 10.0, (current - 30000u16) as f32 / 10.0))
}