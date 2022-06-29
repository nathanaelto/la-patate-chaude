extern crate core;
extern crate core;

mod recover_secret;

use core::panicking::panic;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time;
use common::models::{Challenge, JsonMessage, Subscribe};
use common::models::JsonMessage::Challenge;

struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub fn new() -> TcpClient {
        let stream_addr = TcpStream::connect("localhost:7878");
        let mut stream = match stream_addr {
            Ok(res) => res,
            Err(err) => panic!("Cannot connect: {err}")
        };
        TcpClient {
            stream
        }
    }

    pub fn sendJsonMessage(&mut self, message: &JsonMessage) -> JsonMessage {
        let message_json = serde_json::to_string(&message).unwrap();
        let message_size = message_json.len() as u32;
        self.stream.write(&message_size.to_be_bytes()).unwrap();
        self.stream.write_all(&message_json.as_bytes()).unwrap();

        let mut size_response_buffer = [0u8; 4];
        self.stream.read_exact(&mut size_response_buffer).unwrap();
        let size: u32 = u32::from_be_bytes(size_response_buffer);
        let mut response_buffer: Vec<u8> = vec![0; size as usize];
        self.stream.read_exact(&mut response_buffer).unwrap();
        let response_as_str = std::str::from_utf8(&response_buffer).unwrap();
        let response : JsonMessage = serde_json::from_str(&response_as_str).unwrap();
        response
    }

    pub fn waiting_message(&mut self) -> JsonMessage {
        let mut size_response_buffer = [0u8; 4];
        self.stream.read_exact(&mut size_response_buffer).unwrap();

        let size: u32 = u32::from_be_bytes(size_response_buffer);
        let mut response_buffer: Vec<u8> = vec![0; size as usize];
        self.stream.read_exact(&mut response_buffer).unwrap();
        let response_as_str = std::str::from_utf8(&response_buffer).unwrap();
        let response : JsonMessage = serde_json::from_str(&response_as_str).unwrap();
        response
    }
}


fn main() {

    let player = std::env::args().nth(1).unwrap();

    let mut tcp_client = TcpClient::new();

    println!("-- Hello --");
    let hello = JsonMessage::Hello;
    let message: JsonMessage = tcp_client.sendJsonMessage(&hello);
    println!("{:?}", message);

    println!("-- Subscribe --");

    let name = String::from(player);
    let subscribe = JsonMessage::Subscribe(Subscribe {name});
    let subscribe_response: JsonMessage = tcp_client.sendJsonMessage(&subscribe);
    println!("{:?}", subscribe_response);

    println!("-- Await PlayerBoard --");
    let board : JsonMessage = tcp_client.waiting_message();
    println!("{:?}", board);

    println!("-- Await Challenge or RoundSummary --");
    let challenge : JsonMessage = tcp_client.waiting_message();
    let mut run = true;
    while &run {
        match &challenge {
            Challenge(c) => {
                match &c {
                    Challenge::MD5HashCash(i) => {

                    }
                }
            }
            JsonMessage::RoundSummary(_) => {

            }
            JsonMessage::EndOfGame(_) => {
                run = false;
            }
            _ => {
                panic!("C'est quoi cette merde que le serveur m'a envoyé");
            }
        }
    }
    println!("{:?}", challenge);


}