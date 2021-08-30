use std::convert::TryInto;
use std::result::Result;
use std::error::Error;


/// return (voltage, current, soc)
/// Reply format:
/// head,  tag, cmd,  len,  data                            , check
/// 0xA5, 0x01, 0x90, 0x08, [b0, b1, b2, b3, b4, b5, b6 ,b7], 0x??
/// [165,    1,  144,    8,  0, 217, 0,  0, 117, 48, 0, 209,  141]
///
pub fn parse_90(buffer: [u8; 13]) -> Result<(f32, f32, f32), Box<dyn Error>> {
    let voltage = u16::from_be_bytes(buffer[4..6].try_into()?);
    let current = u16::from_be_bytes(buffer[8..10].try_into()?);
    let soc = u16::from_be_bytes(buffer[10..12].try_into()?);
    Ok((voltage as f32 / 10.0, (current as f32 - 30000.0) / 10.0, soc as f32 / 10.0))
}