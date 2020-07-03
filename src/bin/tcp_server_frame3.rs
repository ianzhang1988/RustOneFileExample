use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read};
use std::convert::TryInto;
use std::str::from_utf8;

use std::mem::swap;


/// using std::men::swap exchange buffer in Frame

fn read_le_u32_simple(input: &[u8]) -> u32 {
    let (int_bytes, _rest) = input.split_at(std::mem::size_of::<u32>());
    u32::from_le_bytes(int_bytes.try_into().unwrap())
}

#[allow(dead_code)]
fn read_le_u32(input: &mut &[u8]) -> u32 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    u32::from_le_bytes(int_bytes.try_into().unwrap())
}


struct Frame {
    header_length: usize,
    body_size : usize,
    // buffer for incoming frame
    data : Vec<u8>,
    data_ready: Option<Vec<u8>>,
}

/// mem swap is easier than refcell in this case
/// are they the same efficiency?

impl Frame {
    fn new() -> Frame {
        Frame{
            header_length:4,
            body_size:0,
            data: Vec::<u8>::new(),
            data_ready:Option::None,
        }
    }

    fn add_data(&mut self, in_data: &[u8]){

        self.data.extend(in_data);

        if self.data.len() > self.header_length {

            // read body size from header
            if  self.body_size < 1 {

                let size_header = &self.data[0..5];
                self.body_size = read_le_u32_simple(&size_header) as usize;

                println!("body size {}", self.body_size);
            }

            // current body length
            let body_length = self.data.len() - self.header_length;

            // one frame finished, maybe next frame appended
            if body_length>= self.body_size {

                let mut swap_data = Vec::<u8>::new();

                // move data from next frame to the head of buffer
                let next_frame_len = self.data.len() - self.body_size - self.header_length;
                if next_frame_len > 0 {
                    println!("next frame data.len {}, body size {}, diff {}", self.data.len(), self.body_size, next_frame_len);
                    let (_left, right) = self.data.split_at(self.body_size);

                    // let mut data = self.data.borrow_mut();
                    // let (_left, right) = data.split_at_mut(self.body_size);

                    swap_data[0..next_frame_len].copy_from_slice(&right);
                }

                // remove data from next frame
                self.data.truncate(self.header_length + self.body_size);

                // let mut data = &self.data;
                swap(&mut self.data, &mut swap_data);

                self.data_ready = Some(swap_data);

                // prepare for next frame
                self.body_size = 0;
            }
        }
    }

    fn get_data(&mut self) -> Option<Vec<u8>> {
        match &mut self.data_ready {
            Some(data) => {
                let mut swap_data = Vec::<u8>::new();

                // need get rid of header ...
                swap(data, &mut swap_data);

                self.data_ready = None;
                Some(swap_data)
            },
            None => None,
        }
    }

}

fn handle_client(mut stream: TcpStream){
    let mut data_frame = Frame::new();

     // using 50 byte as read buffer
    let mut buffer = [0 as u8; 50];

    while match stream.read(&mut buffer) {
        Ok(size) => match size {
            0 => {
                // connection close
                println!("size 0, close");
                false
            },
            _ => {

                data_frame.add_data(&buffer[0..size]);

                if let Some(data) = data_frame.get_data() {
                    let string_data = data;
                    // header still in data, hmm........
                    let text = from_utf8(&string_data[4..]).unwrap();
                    println!("received data: {}", &text);
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