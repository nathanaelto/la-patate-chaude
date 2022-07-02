use std::ops::ControlFlow;
use serde::{Deserialize, Serialize};
use md5::compute;
use crate::challenge::IChallenge;
use crate::md5_checker::Md5Checker;

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

impl IChallenge for MD5HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String { String::from("MD5HashCash") }

    fn new(input: Self::Input) -> Self {
        MD5HashCash {
            input,
            output: MD5HashCashOutput { seed: 0, hashcode: "".to_string() }
        }
    }

    fn solve(&self) -> Self::Output {
        let message = self.input.message.clone();

        let mut seed: u64 = 0;
        let mut output: MD5HashCashOutput;
        loop {
            let seed_str = format!("{:016X}", seed);
            let hashcode  = compute(seed_str + &message);
            let md5 = format!("{:032X}", hashcode);

            output = MD5HashCashOutput {
                seed,
                hashcode: md5.clone()
            };
            if self.verify(MD5HashCashOutput {
                seed,
                hashcode: md5
            }) {
                println!("SEED : {}", seed);
                break;
            }

            seed +=1;
        }
        return output
    }

    fn verify(&self, answer: Self::Output) -> bool {
        let checker : Md5Checker = Md5Checker::new();
        let md5 = answer.hashcode.clone();
        let mut nb_bytes_to0 = 0;
        let chars: Vec<char> = md5.chars().collect();

        chars
            .iter()
            .for_each(|letter| {
                let nb_zero: u32 = checker.get_bits_to_zero(letter.to_string()) ; //tester lettre avec checker
                nb_bytes_to0 += nb_zero;

                if nb_zero < 4 {
                   return;
                }
            });
        // for letter in md5.chars() {
        //     let nb_zero: u32 = checker.get_bits_to_zero(letter.to_string()) ; //tester lettre avec checker
        //     nb_bytes_to0 += nb_zero;
        //
        //     if nb_zero < 4 {
        //         break;
        //     }
        // }
        return nb_bytes_to0 >= self.input.complexity;
    }
}
