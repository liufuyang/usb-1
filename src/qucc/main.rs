use serialport::available_ports;
use usb_1::qucc::QuccBMS;

fn main() {
    match available_ports() {
        Ok(it) => println!("{:?}", it),
        Err(e) => println!("{}", e),
    }

    let mut bms = QuccBMS::new("/dev/tty.usbserial-110").unwrap();
    let vec = bms.get_cell_v().unwrap();
    println!("{:?}", vec);

    let info = bms.get_info().unwrap();
    println!("{:?}", info);

    let info2 = bms.get_info2().unwrap();
    println!("{:?}", info2);
}
