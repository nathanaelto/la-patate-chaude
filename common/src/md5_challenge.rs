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
        //entier de 64 bits
        let mut seed: u64 = 0;
        let mut found = false;

        while !found {
            //formater seed pour qu'il soit en hexa sur 32: println!("{:#01x}", seed);
            let hashcode  = md5::compute(seed+message);
            let mut nbBytesTo0 = 0;
            for &letter in hashcode {
                let nbZero = checker(letter); //tester lettre avec checker
                nbBytesTo0 += nbZero;

                if nbZero < 4 {
                    break;
                }
            }

            if nbBytesTo0 >= self.input.complexity {
                return MD5HashCashOutput {
                    seed,
                    hashcode: String::from(hashcode)
                };
            }

            seed +=1;
        }
        return *self.output
    }

    fn verify(&self, answer: Self::Output) -> bool {
        todo!()
    }
}
