use serde::{Deserialize, Serialize};
use crate::md5_challenge::{MD5HashCashInput, MD5HashCashOutput};

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64,
}

pub struct PublicLeaderBoard {
    public_leader_board: Vec<PublicPlayer>,
}

pub struct Subscribe {
    name: String,
}

pub struct Hello {
}

pub struct Welcome {
    version: u8
}

pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}

pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
}

pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
}

pub struct ChallengeResult {
    answer: ChallengeAnswer,
    next_target: String,
}

pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String },
}

pub struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue,
}

pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>,
}
