use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read};
use std::convert::TryInto;
use std::str::from_utf8;

// use std::rc::Rc;
// use std::cell::Cell;
// use std::borrow::{BorrowMut, Borrow};
// use std::mem::swap;

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


// todo: replace the use of `mut self`, in this scene it do not look good
struct Frame {
    header_length: usize,
    body_size : usize,
    // buffer for incoming frame
    data : Vec<u8>,
    data_ready: Option<Vec<u8>>,
}

impl Frame {
    fn new() -> Frame {
        Frame{
            header_length:4,
            body_size:0,
            data: Vec::<u8>::new(),
            data_ready:Option::None,
        }
    }

    fn add_data(mut self, in_data: &[u8]) -> Self {
        self.data.extend(in_data);


        if self.data.len() > self.header_length {

            // read body size from header
            if  self.body_size < 1 {

                let size_header = &(self.data)[0..5];
                self.body_size = read_le_u32_simple(&size_header) as usize;

                println!("body size {}", self.body_size);
            }

            // current body length
            let body_length = self.data.len() - self.header_length;

            // one frame finished, maybe next frame appended
            if body_length>= self.body_size {

                let mut swap_data = Vec::<u8>::new();


                // get text out of body
                // let body = &self.data[self.header_length..self.body_size + self.header_length];
                // let text = from_utf8(&body).unwrap();
                // println!("received data: {}", &text);

                // move data from next frame to the head of buffer
                let next_frame_len = self.data.len() - self.body_size - self.header_length;
                if next_frame_len > 0 {
                    println!("next frame data.len {}, body size {}, diff {}", self.data.len(), self.body_size, next_frame_len);
                    let (_left, right) = self.data.split_at_mut(self.body_size);
                    swap_data[0..next_frame_len].copy_from_slice(&right);
                }

                // remove data from next frame
                self.data.truncate(self.header_length + self.body_size);
                self.data_ready = Some(self.data);

                // add_data(&mut self, in_data: &[u8])
                // self.data_ready = Some(self.data);
                // move occurs because `self.data` has type `std::vec::Vec<u8>`, which does not implement the `Copy` trait
                // cannot move out of `self.data` which is behind a mutable reference
                // https://stackoverflow.com/questions/35649968/how-to-swap-two-fields-of-a-struct
                //
                // we can use std::men::swap
                // swap(&mut swap_data, &mut self.data);
                // self.data_ready = Some(swap_data);
                //
                // or other way for this
                // https://stackoverflow.com/questions/28258548/cannot-move-out-of-borrowed-content-when-trying-to-transfer-ownership
                // "You cannot transfer ownership of something borrowed because you don't own it"
                // add_data(mut self, in_data: &[u8])
                // instead reference of self, just use self
                // need to return self, for use later, or self would be deconstruct
                //
                // may be Cell<Box<..>>


                self.data = swap_data;

                // prepare for next frame
                self.body_size = 0;
            }
        }

        self

    }

    fn process_data(mut self, func : fn(&[u8])) -> Self {
        // if let Some(ready_data) = self.data_ready {
        //     func(ready_data[self.header_length..])
        // }
        // error: ready_data moved form self, partial moved, if self is returned, no partial move
        // is allowed.
        if let Some(ready_data) = &self.data_ready {
            func(&ready_data[self.header_length..]);
            // but assign a value is fine ???
            self.data_ready = Option::None;
        }

        self
    }
}

fn print_data(data: &[u8]) {
    let text = from_utf8(&data).unwrap();
    println!("received data: {}", &text);
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

                data_frame = data_frame.add_data(&buffer[0..size]);
                data_frame = data_frame.process_data(print_data);
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