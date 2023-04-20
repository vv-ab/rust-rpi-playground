// uart_blocking_read.rs - Blocks while waiting for incoming serial data.

use std::error::Error;
use std::time::Duration;

use rppal::uart::{Parity, Uart};

fn main() -> Result<(), Box<dyn Error>> {
    let mut uart = Uart::new(9600, Parity::None, 8, 1)?;

    uart.set_read_mode(1, Duration::from_secs(1))?;

    let mut buffer = [0u8; 1024];
    loop {

        let bytes_received = match uart.read(&mut buffer) {
            Ok(bytes) => {
                bytes
            }
            Err(a) => {
                println!("error while reading from uart: {:?}", a);
                continue
            }
        };

        println!("received {} bytes", bytes_received);

        if bytes_received > 0 {
            let message = String::from_utf8(Vec::from(buffer))?;
            println!("Received message: {}", message);
        }
    }
}