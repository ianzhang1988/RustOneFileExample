use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read};
use std::convert::TryInto;
use std::str::from_utf8;

// use std::rc::Rc;
use std::cell::RefCell;
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


/// Cell RefCell Interior Mutability
/// Cell gives the value, can get set copy
/// RefCell gives ref to value, swap or replace
///
/// https://www.reddit.com/r/rust/comments/755a5x/i_have_finally_understood_what_cell_and_refcell/
/// > They are both just ways to lie to the compiler. Cell is a way to pretend that a value is immutable and then treat it as mutable.
/// It uses unsafe to mutate the insides. This is needed for cases when the compiler cannot prove that a value is mutated safely.
///
/// > RefCell is a runtime borrow-checker. It's a struct of a T and an additional borrow counter.
/// The borrow counter is either the number of borrows, or 0xFFFFFFFF when there is a mutable reference.
/// When we use the method borrow(), the borrow count increases,
/// and the returned Ref has a pointer to the RefCell's borrow count to decrease it on delete.
/// Same for borrow_mut() and RefMut except that it sets the borrow count to 0 and 0xFFFFFFFF respectively.
///
/// ```
/// let foo = RefCell<T>
/// let foo_ref = foo.borrow()
/// ```
/// access of change T must use foo_ref or runtime borrow-checker maybe failed
/// hence data_ready must Option<RefCell<Vec<u8>>> (also use RefCell), It's like a lock.
///


/// maybe unsafe is the easy way to accomplish this

struct Frame {
    header_length: usize,
    body_size : usize,
    // buffer for incoming frame
    data : RefCell<Vec<u8>>,
    data_ready: Option<RefCell<Vec<u8>>>,
}


impl Frame {
    fn new() -> Frame {
        Frame{
            header_length:4,
            body_size:0,
            data: RefCell::new(Vec::<u8>::new()),
            data_ready:Option::None,
        }
    }

    fn add_data(&mut self, in_data: &[u8]){
        // self.data.extend(in_data);

        self.data.borrow_mut().extend(in_data);


        if self.data.borrow().len() > self.header_length {

            // read body size from header
            if  self.body_size < 1 {

                let size_header = &self.data.borrow()[0..5];
                self.body_size = read_le_u32_simple(&size_header) as usize;

                println!("body size {}", self.body_size);
            }

            // current body length
            let body_length = self.data.borrow().len() - self.header_length;

            // one frame finished, maybe next frame appended
            if body_length>= self.body_size {

                let mut swap_data = Vec::<u8>::new();


                // get text out of body
                // let body = &self.data[self.header_length..self.body_size + self.header_length];
                // let text = from_utf8(&body).unwrap();
                // println!("received data: {}", &text);

                // move data from next frame to the head of buffer
                let next_frame_len = self.data.borrow().len() - self.body_size - self.header_length;
                if next_frame_len > 0 {
                    println!("next frame data.len {}, body size {}, diff {}", self.data.borrow().len(), self.body_size, next_frame_len);
                    let data = self.data.borrow();
                    let (_left, right) = data.split_at(self.body_size);

                    // let mut data = self.data.borrow_mut();
                    // let (_left, right) = data.split_at_mut(self.body_size);

                    swap_data[0..next_frame_len].copy_from_slice(&right);
                }

                // remove data from next frame
                self.data.borrow_mut().truncate(self.header_length + self.body_size);

                let swap_cell = RefCell::new(swap_data);
                swap_cell.swap(&self.data);

                self.data_ready = Some(swap_cell);

                // prepare for next frame
                self.body_size = 0;
            }
        }
    }

    fn get_data(&mut self) -> Option<RefCell<Vec<u8>>> {
        match &self.data_ready {
            Some(data) => {
                let swap = RefCell::new(Vec::<u8>::new());

                // need get rid of header ...
                swap.swap(data);

                self.data_ready = None;
                Some(swap)
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
                    let string_data = data.borrow();
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