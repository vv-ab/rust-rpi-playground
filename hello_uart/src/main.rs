// uart_blocking_read.rs - Blocks while waiting for incoming serial data.

use std::error::Error;
use std::io;
use std::io::BufRead;
use std::mem::size_of;
use std::time::Duration;

use rppal::uart::{Parity, Uart};
use serde::de::Unexpected::Str;

#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq)]
struct Data {
    id: u32,
    //message: &'a str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut uart = Uart::new(9600, Parity::None, 8, 1)?;

    uart.set_read_mode(1, Duration::from_secs(1))?;
    uart.set_write_mode(false)?;

    let mut buffer = [0u8; size_of::<Data>()];

    loop {

        {
            println!("Press enter.");
            let stdin = io::stdin();
            let mut line = String::new();
            stdin.read_line(&mut line)?;
        }

        println!("Sending command.");
        match uart.write(&[1, 211]) {
            Ok(_) => {
                println!("Command sent successfully.");
            }
            Err(_) => {
                println!("sent command failed.");
            }
        }

        let bytes_received = match uart.read(&mut buffer) {
            Ok(bytes) => {
                println!("Received measurement");
                bytes
            }
            Err(a) => {
                println!("error while reading from uart: {:?}", a);
                continue;
            }
        };

        if bytes_received > 0 {
            let message_length = buffer[0] as usize;
            let mut message: Vec<u8> = Vec::with_capacity(message_length);
            let mut remaining_bytes = message_length;
            while remaining_bytes > 0 {
                let bytes_read = uart.read(&mut buffer).unwrap();
                remaining_bytes -= bytes_read;
                message.extend(buffer.iter().take(bytes_read));
            }
            println!("Message: {message:?}");
            match postcard::from_bytes::<Data>(message.as_slice()) {
                Ok(data) => {
                    println!("Successfully deserialized: {data:?}");
                }
                Err(err) => {
                    println!("failed to deserialize bytes into data: {err:?}");
                }
            };

            for i in 0..buffer.len() {
                buffer[i] = 0;
            }
        }
    }
}
