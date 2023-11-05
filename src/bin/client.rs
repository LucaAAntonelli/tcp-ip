use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap_or(0);

            let msg = input.as_bytes();

            stream.write(msg).unwrap();
            println!("Sent message, awaiting reply...");

            let mut data = [0 as u8]; // using 6 byte buffer
            match stream.read(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
