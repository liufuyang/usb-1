use crc16::{State, MODBUS};
use serialport::{available_ports, SerialPort};
use std::io::{Read, Write};
use std::time::Duration;

fn main() {
    match available_ports() {
        Ok(it) => println!("{:?}", it),
        Err(e) => println!("{}", e),
    }

    let mut port = serialport::new("/dev/tty.usbserial-110", 9600)
        .timeout(Duration::from_secs(2))
        .open_native()
        .expect("Failed to open port");

    println!("timeout: {}", port.timeout().as_secs());

    let mut decoded = hex::decode("010310180003").expect("Decoding failed");
    let ck = State::<MODBUS>::calculate(decoded.as_slice());
    decoded.append(&mut ck.to_le_bytes().to_vec());
    println!("{}", hex::encode(decoded.clone()));

    // Write format:
    // head,  tag, cmd,  len,  data                            , check
    // 0xA5, 0x80, 0x90, 0x08, [b0, b1, b2, b3, b4, b5, b6 ,b7], 0x??
    // [165,   80,  144,    8,  0,  0,  0,  0,  0,  0,  0,  0, 141], xx
    port.write_all(decoded.as_slice());

    // Reply format:
    let mut buffer = [0; 11];
    let i = port.read(&mut buffer).expect("read failed");
    println!("read: {}", i);
    println!("{}", hex::encode(buffer));

    // let soc_result = usb_1::qucc::parse_cell_voltage(buffer).unwrap();
    // println!(
    //     "voltage: {}V, current: {}A, soc: {}%",
    //     soc_result.0, soc_result.1, soc_result.2
    // );
}
