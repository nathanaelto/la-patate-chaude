
mod recover_secret;

use std::io::{Read, Write};
use std::net::TcpStream;
use rand::{Rng};
use common::md5_challenge::{MD5HashCash, MD5HashCashOutput};
use common::models::{Challenge, ChallengeAnswer, ChallengeResult, JsonMessage, PublicPlayer, Subscribe};
use common::challenge::IChallenge;

struct TcpClient {
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
    let message: JsonMessage = tcp_client.send_and_await_json_message(&hello);
    println!("{:?}", message);

    println!("-- Subscribe --");
    let name = String::from(player.clone());
    let subscribe = JsonMessage::Subscribe(Subscribe {name});
    let subscribe_response: JsonMessage = tcp_client.send_and_await_json_message(&subscribe);
    println!("{:?}", subscribe_response);

    println!("-- Await PlayerBoard --");
    let board_response : JsonMessage = tcp_client.waiting_message();
    let players: &Vec<PublicPlayer>;
    match &board_response {
        JsonMessage::PublicLeaderBoard(p) => {
            players = &*p;
        }
        _ => {
            panic!("Il y a un pb là non ?")
        }
    }


    println!("{:?}", players);

    let mut rng = rand::thread_rng();

    println!("-- Await Challenge or RoundSummary --");

    loop {
        let record = tcp_client.waiting_message();
        match record {
            JsonMessage::Challenge(challenge) => {
                let mut target_index = rng.gen_range(0..players.len());
                let mut target: &PublicPlayer = players.get(target_index).unwrap();
                while target.name == player {
                    target_index = rng.gen_range(0..players.len());
                    target = players.get(target_index).unwrap();
                }
                match challenge {
                    Challenge::MD5HashCash(input) => {
                        // let i : MD5HashCashInput = input;
                        let cha: MD5HashCash = MD5HashCash::new(input);
                        let res: MD5HashCashOutput = cha.solve();
                        tcp_client.send_json_message(
                            &JsonMessage::ChallengeResult(ChallengeResult {
                                next_target: target.name.clone(),
                                answer: ChallengeAnswer::MD5HashCash(res)
                            })
                        )
                    }

                }


            }
            JsonMessage::RoundSummary(round_summary) => {
                println!("{:?}", round_summary);
            }
            JsonMessage::PublicLeaderBoard(board) => {
                println!("{:?}", board);
            }
            JsonMessage::EndOfGame(_) => {
                println!("{:?}", record);
                break;
            }
            _ => {
                panic!("C'est quoi cette merde que le serveur m'a envoyé");
            }
        }
    }
}