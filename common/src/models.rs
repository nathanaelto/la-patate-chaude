use core::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::challenge::md5_challenge::{MD5HashCashInput, MD5HashCashOutput};

#[derive(Serialize, Debug)]
pub struct Hello {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome {
    pub version: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JsonMessage {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

// #[derive(Serialize, Deserialize, Debug, Display)]
// #[display(fmt = "Player : {} - {}\nScore : {}\nStep : {}\nIsActive : {}\nTotal time used : {}\n", name, stream_id, score, steps, is_active, total_used_time)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64,
}

impl fmt::Display for PublicPlayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player : {} - {}\nScore : {}\nStep : {}\nIsActive : {}\nTotal time used : {}\n",
            self.name, self.stream_id, self.score, self.steps, self.is_active, self.total_used_time
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicLeaderBoard {
    pub public_leader_board: Vec<PublicPlayer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundSummary {
    pub challenge: String,
    pub chain: Vec<ReportedChallengeResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndOfGame {
    leader_board: Vec<PublicPlayer>,
}
