

use std::io::Read;
use std::net::{SocketAddr, TcpListener};
use common::challenge::Challenge;
use common::models;
use common::models::SubscribeError;
use crate::recover_secret::recover_secret::{RecoverSecret, RecoverSecretInput};

fn main() {
    let subscribe_error = SubscribeError::InvalidName;
    let serialized_error = serde_json::to_string(&subscribe_error).unwrap();

    //phrase à trouver : il fait froid
    // let input = RecoverSecretInput{
    //     letters: String::from("lffiiilfatroridato"),
    //     tuple_sizes: Vec::from([3,3,3,3,3,3])
    // };
    //
    // let challenge = RecoverSecret::new(input);

    println!("{}", serialized_error);

    //TODO déplacer dans un fichier
    //Server
    let addr: SocketAddr = SocketAddr::from(([127,0,0,1], 7676));
    let x = TcpListener::bind(addr);
    // let listener = x.unwrap(); // mode 'bourrain'

    let listener = match x {
        Ok(res) => res,
        Err(err) => panic!("Cannot listen on port : {err:?}")
    };

    for message in listener.incoming() {
        println!("message entrant: {message:?}");
        let mut decoded = message.unwrap();
        let mut v = Vec::<u8>::new();
        decoded.read_to_end(&mut v).unwrap();

        let str = String::from_utf8_lossy(&v);
        println!("{str:?}");
        println!("{str:?}");
    }
}
