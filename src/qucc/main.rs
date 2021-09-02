use crc16::{State, MODBUS};
use serialport::{available_ports, SerialPort};
use std::io::{Read, Write};
use std::time::Duration;
use usb_1::qucc::QuccBMS;

fn main() {
    match available_ports() {
        Ok(it) => println!("{:?}", it),
        Err(e) => println!("{}", e),
    }

    let mut bms = QuccBMS::new("/dev/tty.usbserial-110", 8);
    let vec = bms.get_cell_v().unwrap();
    println!("{:?}", vec);
}
