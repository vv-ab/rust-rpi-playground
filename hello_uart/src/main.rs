// uart_blocking_read.rs - Blocks while waiting for incoming serial data.

use std::error::Error;
use std::mem::size_of;
use std::time::Duration;

use rppal::uart::{Parity, Uart};

#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq)]
struct Data {
    id: u32,
    //message: &'a str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut uart = Uart::new(9600, Parity::None, 8, 1)?;

    uart.set_read_mode(size_of::<Data>() as u8, Duration::from_secs(1))?;

    let mut buffer = [0u8; size_of::<Data>()];

    loop {
        let bytes_received = match uart.read(&mut buffer) {
            Ok(bytes) => {
                bytes
            }
            Err(a) => {
                println!("error while reading from uart: {:?}", a);
                continue;
            }
        };

        println!("received {} bytes", bytes_received);

        if bytes_received > 0 {
            let answer = buffer[0].wrapping_add(1);

            match postcard::from_bytes::<Data>(&buffer) {
                Ok(data) => {
                    println!("Successfully deserialized bytes into data: {data:?}");
                }
                Err(err) => {
                    println!("failed to deserialize bytes into data: {err:?}");
                }
            };

            println!("Received message: {}, Answering with: {}", buffer[0], answer);

            match uart.write(&[answer, 113u8]) {
                Ok(bytes) => {
                    println!("Successfully sent {bytes} bytes");
                }
                Err(err) => {
                    println!("error while transferring: {err:?}");
                }
            };
        }
    }
}
