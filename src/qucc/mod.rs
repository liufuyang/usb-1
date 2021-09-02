use crate::error::LibError;
use crc16::{State, MODBUS};
use serialport::TTYPort;
use std::convert::TryInto;
use std::error::Error;
use std::io::{Read, Write};
use std::time::Duration;

pub struct QuccBMS {
    port: TTYPort,
    cell_count: u16,
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
        QuccBMS { port, cell_count }
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
