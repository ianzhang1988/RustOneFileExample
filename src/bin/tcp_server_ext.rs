use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::convert::TryInto;
use std::str::from_utf8;

fn read_le_u32_simple(input:  &[u8]) -> u32 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    u32::from_le_bytes(int_bytes.try_into().unwrap())
}

fn read_le_u32(input: &mut &[u8]) -> u32 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    u32::from_le_bytes(int_bytes.try_into().unwrap())
}

fn handle_client(mut stream: TcpStream){
    let mut header_leanth: usize = 4;
    let mut body_size = 0;
    let mut data = Vec::<u8>::new();

    println!("data len {}", &data.len());


    let mut buffer = [0 as u8; 50]; // using 50 byte buffer

    while match stream.read(&mut buffer) {
        Ok(size) => match size {
            0 => {
                println!("size 0, close");
                false
            },
            _ => {
                // echo everything!
                // stream.write(&buffer[0..size]).unwrap();
                println!("size {}", &size);
                println!("buffer {:?}", buffer.clone().to_vec());
                println!("buffer len {}", buffer.len());

                data.extend(&buffer[0..size]);
                // data[buf_pos..].copy_from_slice(&buffer[0..size]);

                if data.len() > header_leanth {

                    if  body_size < 1 {
                        let size_header = &data[0..5];
                        body_size = read_le_u32_simple(&size_header) as usize;

                        println!("body size {}", body_size);
                    }

                    let body_length = data.len() - header_leanth;

                    if  body_length>= body_size {
                        // one frame finished

                        let body = &data[header_leanth..body_size+header_leanth];
                        let text = from_utf8(&body).unwrap();
                        println!("received data: {}", &text);


                        let next_frame_len = data.len() - body_size - header_leanth;
                        if next_frame_len > 0 {
                            println!("next frame data.len {}, body size {}, diff {}", data.len(), body_size, next_frame_len);
                            let (left, right) = data.split_at_mut(body_size);
                            left[0..next_frame_len].copy_from_slice(&right);
                        }

                        data.truncate(next_frame_len);
                        data.shrink_to_fit();

                        body_size=0;
                    }
                }

                true
            }
        },
        Err(e) => {
            println!("Error: {}", e);
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:5555").unwrap();
    println!("server started");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }

    // close the socket server
    drop(listener);
}