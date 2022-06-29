extern crate core;

mod recover_secret;

use std::io::{BufWriter, Read, Write};
use std::net::TcpStream;
use common::models::{Hello, JsonMessage, Subscribe, Welcome};

//TODO à compléter/améliorer
fn main() {
    let stream_addr = TcpStream::connect("localhost:7878");

    let mut stream = match stream_addr {
        Ok(res) => res,
        Err(err) => panic!("Cannot connect: {err}")
    };

    // let hello = Hello {};
    // println!("{:?}", hello);
    // let hello_json = serde_json::to_string(&hello);
    // println!("{:?}", hello_json);

    // println!("-- String and byte --");
    // let hello = String::from("Hello");
    // println!("{}", hello);
    // println!("{}", hello.len());
    // let hello_byte = hello.as_bytes();
    // println!("{:?}", hello_byte);
    // println!("{}", hello_byte.len());
    //
    // println!("-- String and json --");
    // let hello_json = serde_json::to_string(&hello).unwrap();
    // let hello_json_size = hello_json.len() as u32;
    // println!("{}", hello_json);
    // println!("{}", hello_json_size);

    println!("-- Hello --");
    let hello = JsonMessage::Hello;
    let hello_json = serde_json::to_string(&hello).unwrap();
    let hello_size = hello_json.len() as u32;
    // println!("{}", hello_json);
    // println!("{}", hello_size);

    stream.write(&hello_size.to_be_bytes()).unwrap();
    stream.write_all(&hello_json.as_bytes()).unwrap();

    let mut size_response = [0u8; 4];
    stream.read_exact(&mut size_response).unwrap();

    let size: u32 = u32::from_be_bytes(size_response);
    let mut data: Vec<u8> = vec![0; size as usize];

    stream.read_exact(&mut data).unwrap();
    let message_str = std::str::from_utf8(&data).unwrap();
    // println!("{:?}", message_str);

    let message: JsonMessage = serde_json::from_str(&message_str).unwrap();
    println!("{:?}", message);

    println!("-- Subscripbe --");

    let name = String::from("Natha");
    let subscribe = JsonMessage::Subscribe(Subscribe {name});
    let subscribe_json = serde_json::to_string(&subscribe).unwrap();
    let subscribe_size = subscribe_json.len() as u32;

    stream.write(&subscribe_size.to_be_bytes()).unwrap();
    stream.write(&subscribe_json.as_bytes()).unwrap();

    let mut subscribe_response_size_buffer =  [0u8; 4];
    stream.read_exact(&mut subscribe_response_size_buffer).unwrap();
    let subscribe_response_size: u32 = u32::from_be_bytes(subscribe_response_size_buffer);
    let mut subscribe_response_buffer: Vec<u8> = vec![0; subscribe_response_size as usize];
    stream.read_exact(&mut subscribe_response_buffer).unwrap();
    let subscribe_response_str = std::str::from_utf8(&subscribe_response_buffer).unwrap();
    let subscribe_response: JsonMessage = serde_json::from_str(&subscribe_response_str).unwrap();
    println!("{:?}", subscribe_response);

}