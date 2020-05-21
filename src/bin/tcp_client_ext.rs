use std::net::{TcpStream};
use std::io::{self, BufRead, Write};

fn main() {
    let mut buffer = String::new();
    let mut trim_buffer="";

    match TcpStream::connect("localhost:5555") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            // loop read stdin and send input to server
            while match trim_buffer {
                "stop" => false, // type stop to exit client
                _ => {
                    buffer.clear();
                    true
                }
            }
            {
                // get input without \n or \r\n
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                handle.read_line(&mut buffer).unwrap();
                trim_buffer = buffer.trim_end();

                println!("input {}", &trim_buffer);

                // string in bytes
                let send_buffer = trim_buffer.as_bytes();

                // len return usize witch is 64 bit in my project
                // I only need u32 hence as u32
                let length = send_buffer.len() as u32;

                // get bytes for u32, socket would translate this to 'be', so keep it here in 'le'
                let size_header = length.to_le_bytes();

                // call me lazy for not use another buffer to put things together
                stream.write(&size_header).unwrap();
                stream.write(send_buffer).unwrap();

                // println!("data sent");
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}