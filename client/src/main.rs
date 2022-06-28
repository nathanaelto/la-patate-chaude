mod recover_secret;

use std::io::{BufWriter, Write};
use std::net::TcpStream;
use common::models::{JsonMessage, Welcome};

//TODO à compléter/améliorer
fn main() {
    println!("Hello");
    let stream = TcpStream::connect("localhost:7878");

    let mut listener = match stream {
    // let listener = x.unwrap(); // mode 'bourrain'
        Ok(res) => res,
        Err(err) => panic!("Cannot connect: {err}")
    };
    let message = &"Hello"[..];
    let json_message = JsonMessage{
        size: 5,
        message: *message,
    };
    let x = listener.send(serde_json::to_string(&json_message));
    // let mut decoded = message.unwrap();
    // let mut v = Welcome();
    // decoded.read_to_end(&mut v).unwrap();
    // println!()

}