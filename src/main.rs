use serialport::available_ports;

fn main() {
    println!("Hello, world! Raspberry Pi!");

    match available_ports() {
        Ok(it) => println!("{:?}", it),
        Err(e) => println!("{}", e)
    }
}
