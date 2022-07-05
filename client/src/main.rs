use rand::distributions::Alphanumeric;
use rand::Rng;
use common::challenge::IChallenge;
use common::md5_challenge::{MD5HashCash, MD5HashCashOutput};
use common::models::{Challenge, ChallengeAnswer, ChallengeResult, JsonMessage, PublicPlayer, Subscribe};
use crate::tcp_client::tpc_client::TcpClient;

mod recover_secret;
mod tcp_client;

fn main() {
    let player: String;
    let name_input = std::env::args().nth(1);
    match name_input {
        None => {
            //Génère un nom aléatoire pour le joueur
            player = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();
        }
        Some(name) => {
            player = name;
        }
    }
    println!("Player's name is {:?}", player);

    let mut tcp_client = TcpClient::new();

    let mut turn: u32 = 0;

    println!("-- Hello --");
    let hello = JsonMessage::Hello;
    tcp_client.send_and_await_json_message(&hello);
    // let message: JsonMessage = tcp_client.send_and_await_json_message(&hello);
    // println!("{:?}", message);

    println!("-- Subscribe --");
    let name = String::from(player.clone());
    let subscribe = JsonMessage::Subscribe(Subscribe {name});
    // let subscribe_response: JsonMessage = tcp_client.send_and_await_json_message(&subscribe);
    tcp_client.send_and_await_json_message(&subscribe);
    // println!("{:?}", subscribe_response);

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

    println!("-- Await Challenge or RoundSummary --");

    loop {
        let record = tcp_client.waiting_message();

        match record {
            JsonMessage::Challenge(challenge) => {
                let mut target: &PublicPlayer = get_target(players);

                while target.name == player {
                    target = get_target(players);
                }

                match challenge {

                    Challenge::MD5HashCash(input) => {
                        // let i : MD5HashCashInput = input;
                        turn += 1;
                        println!("TURN : {}", turn);
                        println!("{:?}", input);
                        let md5_challenge: MD5HashCash = MD5HashCash::new(input);
                        let md5_solution: MD5HashCashOutput = md5_challenge.solve();
                        let response: JsonMessage = JsonMessage::ChallengeResult(
                            ChallengeResult {
                                next_target: target.name.clone(),
                                answer: ChallengeAnswer::MD5HashCash(md5_solution)
                            }
                        );
                        tcp_client.send_json_message(
                            &response
                        );
                        // println!("\n-- Challenge resolve \n--");
                        println!("{:?}", response);
                    }

                }

            }
            JsonMessage::RoundSummary(round_summary) => {
                println!("{:?}", round_summary.chain);
                // println!("round_summary");
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

fn get_target(players: &Vec<PublicPlayer>) -> &PublicPlayer {
    let mut rng = rand::thread_rng();
    let target_index = rng.gen_range(0..players.len());
    let target: &PublicPlayer;

    match players.get(target_index) {
        None => {
            panic!("Le joueur à disparu !?");
        }
        Some(x) => {
            target = x;
        }
    }


    return target; //Option<PublicPlayer>

}