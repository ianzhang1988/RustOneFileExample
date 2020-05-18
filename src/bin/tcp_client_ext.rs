use std::net::{TcpStream};
use std::io::{self, Read, Write};
use std::str::from_utf8;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();

    let send_buffer = buffer.as_bytes();

    let length = send_buffer.len();

    let size_header = length.to_be_bytes();

    match TcpStream::connect("localhost:5555") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            stream.write(size_header).unwrap();
            stream.write(send_buffer).unwrap();

            println!("data sent");

            // let mut data = [0 as u8; 6]; // using 6 byte buffer
            // match stream.read_exact(&mut data) {
            //     Ok(_) => {
            //         if &data == msg {
            //             println!("Reply is ok!");
            //         } else {
            //             let text = from_utf8(&data).unwrap();
            //             println!("Unexpected reply: {}", text);
            //         }
            //     },
            //     Err(e) => {
            //         println!("Failed to receive data: {}", e);
            //     }
            // }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");





}