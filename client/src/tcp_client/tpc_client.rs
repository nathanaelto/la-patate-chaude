use std::io::{Read, Write};
use std::net::TcpStream;
use common::models::JsonMessage;

pub struct TcpClient {
    stream: TcpStream,
}


impl TcpClient {
    pub fn new() -> TcpClient {
        let stream_addr = TcpStream::connect("localhost:7878");
        let stream = match stream_addr {
            Ok(res) => res,
            Err(err) => panic!("Cannot connect: {err}")
        };
        TcpClient {
            stream
        }
    }

    pub fn send_json_message(&mut self, message: &JsonMessage) {
        let message_json = serde_json::to_string(&message).unwrap();
        let message_size = message_json.len() as u32;
        self.stream.write(&message_size.to_be_bytes()).unwrap();
        self.stream.write_all(&message_json.as_bytes()).unwrap()
    }

    pub fn send_and_await_json_message(&mut self, message: &JsonMessage) -> JsonMessage {
        self.send_json_message(message);
        self.waiting_message()
    }

    pub fn waiting_message(&mut self) -> JsonMessage {
        let mut size_response_buffer = [0u8; 4];
        // println!("-- Read size --");
        loop {
            let res: Result<(), _> = self.stream.read_exact(&mut size_response_buffer);
            if res.is_ok() {
                break;
            }
        }

        let size: u32 = u32::from_be_bytes(size_response_buffer);
        let mut response_buffer: Vec<u8> = vec![0; size as usize];
        // println!("-- Read json --");
        self.stream.read_exact(&mut response_buffer).unwrap();
        let response_as_str = std::str::from_utf8(&response_buffer).unwrap();
        let response : JsonMessage = serde_json::from_str(&response_as_str).unwrap();
        response
    }
}