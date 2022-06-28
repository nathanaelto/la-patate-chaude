mod recover_secret;

use std::io::{BufWriter, Write};
use std::net::TcpStream;

//TODO à compléter/améliorer
fn main() {
    println!("Hello");
    let stream = TcpStream::connect("localhost:7878");

    let mut listener = match stream {
    // let listener = x.unwrap(); // mode 'bourrain'
        Ok(res) => res,
        Err(err) => panic!("Cannot connect: {err}")
    };
    let message = "Hello".as_bytes();

    let x = listener.write(message);
}