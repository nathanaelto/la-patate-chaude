use serde::{Deserialize, Serialize};
use md5::*;
use std::io;
use crate::challenge::Challenge;

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}

pub struct MD5HashCash {
    pub input: MD5HashCashInput,
    pub output: MD5HashCashOutput,
}

impl Challenge for MD5HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String { String::from("MD5HashCash") }

    fn new(input: Self::Input) -> Self {
        todo!()
    }

    fn solve(&self) -> Self::Output {
        // Trouver seed telle que MD5 de seed + message égal à hashcode.
        // Il faut aussi trouver le hashcode ... de ses morts

        // Avec hashcode comprenant X bits à 0 où X supérieur ou égal à complexity.

        // Askip c'est le nombre de bit à 0 en partant de la gauche qu'il faut compter.
        let &message = self.input.message;
        let mut seed = "0000000000000000";
        let hashcode  = md5::compute(seed+message);
        let mut nbBytesTo0 = 0;

        // echo -n "000000000000034Chello" | md5sum | tr a-z A-Z
        // compter le nombre de 0 en partant de la gauche + 1er char non 0
        // Convertir en nb bytes à 0
        // Comparer à complexity
        return *self.output
    }

    fn verify(&self, answer: Self::Output) -> bool {
        todo!()
    }
}
