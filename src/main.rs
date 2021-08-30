use serialport::{available_ports, SerialPort};
use std::io::{Read, Write};
use std::time::Duration;

fn main() {
    println!("Hello, world! Raspberry Pi!");

    match available_ports() {
        Ok(it) => println!("{:?}", it),
        Err(e) => println!("{}", e)
    }

    let mut port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_secs(2))
        .open_native().expect("Failed to open port");

    println!("timeout: {}", port.timeout().as_secs());

    let decoded = hex::decode("a58090080000000000000000bd").expect("Decoding failed");
    println!("{:?}", decoded);

    // Write format:
    // head,  tag, cmd,  len,  data                            , check
    // 0xA5, 0x80, 0x90, 0x08, [b0, b1, b2, b3, b4, b5, b6 ,b7], 0x??
    // [165,   80,  144,    8,  0,  0,  0,  0,  0,  0,  0,  0, 141], xx
    port.write_all(decoded.as_slice());

    // Reply format:
    // head,  tag, cmd,  len,  data                            , check
    // 0xA5, 0x01, 0x90, 0x08, [b0, b1, b2, b3, b4, b5, b6 ,b7], 0x??
    // [165,    1,  144,    8,  0, 217, 0,  0, 117, 48, 0, 209,  141]
    let mut buffer = [0; 13];
    let i = port.read(&mut buffer).expect("read failed");
    println!("read: {}", i);
    println!("{:?}", buffer);

    let soc_result = usb_1::parse_90(buffer).unwrap();
    println!("voltage: {}V, current: {}A, soc: {}%", soc_result.0, soc_result.1, soc_result.2);
}
