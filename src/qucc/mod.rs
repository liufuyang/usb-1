use crate::error::LibError;
use crc16::{State, MODBUS};
use serialport::TTYPort;
use std::convert::TryInto;
use std::error::Error;
use std::io::{Read, Write};
use std::time::Duration;

pub struct QuccBMS {
    device: String,
    port: TTYPort,
    cell_count: u16,
}

#[derive(Debug)]
pub struct Info {
    cell_number: u16,
    running_time: u16,
    soh: u16,
    voltage: f32,
    current: f32,
    temperature: Vec<f32>,
}

const AD: &'static str = "01"; // client address
const FN_R: &'static str = "03"; // Function code: read and hold
const FN_W: &'static str = "06"; // Function code: write save register

impl QuccBMS {
    pub fn new(device: &str, cell_count: u16) -> QuccBMS {
        let mut port = serialport::new(device, 9600)
            .timeout(Duration::from_secs(2))
            .open_native()
            .expect("Failed to open port");
        QuccBMS {
            device: device.to_string(),
            port,
            cell_count,
        }
    }

    pub fn get_device(&self) -> &str {
        self.device.as_str()
    }

    pub fn get_info(&mut self) -> Result<Info, Box<dyn Error>> {
        let reg_start_add = "1000";
        let number_to_read = hex::encode(8u16.to_be_bytes());
        let mut decoded =
            hex::decode(format!("{}{}{}{}", AD, FN_R, reg_start_add, number_to_read))?;
        let ck = State::<MODBUS>::calculate(decoded.as_slice());
        decoded.append(&mut ck.to_le_bytes().to_vec());
        println!("{}", hex::encode(decoded.clone()));

        self.port.write_all(decoded.as_slice());

        let mut buffer = [0; 32];
        let i = self.port.read(&mut buffer)?;
        println!("read: {}", i);
        println!("{}", hex::encode(buffer));
        crc16_verify(&buffer[0..i]);

        Ok(Info::from_bytes(&buffer[3..3 + 16].try_into().unwrap()))
    }

    pub fn get_cell_v(&mut self) -> Result<Vec<u16>, Box<dyn Error>> {
        let reg_start_add = "1017";
        let number_to_read = hex::encode(self.cell_count.to_be_bytes());
        let mut decoded =
            hex::decode(format!("{}{}{}{}", AD, FN_R, reg_start_add, number_to_read))?;
        let ck = State::<MODBUS>::calculate(decoded.as_slice());
        decoded.append(&mut ck.to_le_bytes().to_vec());
        println!("{}", hex::encode(decoded.clone()));

        self.port.write_all(decoded.as_slice());

        let mut buffer = [0; 32];
        let i = self.port.read(&mut buffer)?;
        println!("read: {}", i);
        println!("{}", hex::encode(buffer));
        crc16_verify(&buffer[0..i]);

        let mut result = Vec::new();
        for i in 0..self.cell_count {
            let index = 3 + i as usize * 2;
            let voltage = u16::from_be_bytes(buffer[index..index + 2].try_into()?);
            result.push(voltage);
        }
        Ok(result)
    }
}

impl Info {
    pub fn from_bytes(bytes: &[u8; 16]) -> Info {
        Info {
            cell_number: u16::from_be_bytes(bytes[0..2].try_into().unwrap()),
            running_time: u16::from_be_bytes(bytes[2..4].try_into().unwrap()),
            soh: u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
            voltage: u16::from_be_bytes(bytes[6..8].try_into().unwrap()) as f32 / 100.0,
            current: u16::from_be_bytes(bytes[8..10].try_into().unwrap()) as f32 * -0.1 + 1000.0,
            temperature: vec![
                u16::from_be_bytes(bytes[10..12].try_into().unwrap()) as f32 * 0.1 - 40.0,
                u16::from_be_bytes(bytes[12..14].try_into().unwrap()) as f32 * 0.1 - 40.0,
                u16::from_be_bytes(bytes[14..16].try_into().unwrap()) as f32 * 0.1 - 40.0,
            ],
        }
    }
}

fn crc16_verify(input: &[u8]) -> Result<(), Box<dyn Error>> {
    let (data, check) = input.split_at(input.len() - 2);
    let ck = State::<MODBUS>::calculate(data);
    if ck != u16::from_be_bytes(check.try_into()?) {
        Err(Box::new(LibError::CheckSumError(
            "Read data crc16 not match".into(),
        )))
    } else {
        Ok(())
    }
}
