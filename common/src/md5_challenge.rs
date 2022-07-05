use std::fmt::Debug;
use std::ops::{ControlFlow};
use std::sync::{Arc, Mutex};
use std::thread;
use md5::compute;
use serde::{Deserialize, Serialize};

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

fn is_valide_hexa(hex: String, expected_dif: u32) -> bool {
    let decimal: u128;
    if let Ok(decimal_from_hex) = u128::from_str_radix(&*hex, 16) {
        decimal = decimal_from_hex
    } else {
        println!("Erreur lors de la vérification du format hexa");
        return false;
    }

    let binaire_size_all = format!("{:0128b}", decimal).len() as u32;
    let binaire_size = format!("{:b}", decimal).len() as u32;

    (binaire_size_all - binaire_size) >= expected_dif
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
        let mutex_for_result = Arc::new((
            Mutex::new(false),
            Mutex::new(MD5HashCashOutput { seed: 0, hashcode: "".to_string() })
        ));
        let mutex_complexity = Arc::new(Mutex::new(self.input.complexity));
        let max_seed : u64 = 18_446_744_073_709_551_615;
        let nb_thread: u64 = 100;
        let loop_of_thread = max_seed / nb_thread;
        let mut threads = Vec::new();

        for i in 0..nb_thread {
            let local_message = message.clone();
            let local_complexity = Arc::clone(&mutex_complexity);
            let thread_mutex = Arc::clone(&mutex_for_result);

            threads.push(thread::spawn(move || {
                let seed: u64 = i * loop_of_thread;
                let (flag, res) = &*thread_mutex;
                for s in seed..(seed + loop_of_thread) {
                    let mut locked_flag;

                    if let Ok(locked) = flag.lock() {
                        locked_flag = locked;
                    } else {
                        continue;
                    }

                    if *locked_flag {
                        break
                    }

                    let seed_str = format!("{:016X}", s.clone());
                    let md5  = compute(seed_str.clone() + &local_message);
                    let hashcode = format!("{:032X}", md5);
                    let locked_complexity;

                    if let Ok (lock) = local_complexity.lock() {
                        locked_complexity = lock;
                    } else {
                        continue;
                    }

                    let is_valid = is_valide_hexa(
                        hashcode.clone(),
                        *locked_complexity
                    );

                    if is_valid {
                        *locked_flag = true;
                        let decimal;

                        if let Ok(decimal_from_hex) = u128::from_str_radix(
                            &*hashcode.clone(),
                            16
                        ) {
                            decimal = decimal_from_hex
                        } else {
                            println!("Erreur lors de la récupération du format décimal");
                            continue;
                        }

                        println!("SEED : {} -> {} : {:?}", s, hashcode, format!("{:0128b}", decimal));

                        *res.lock().unwrap() = MD5HashCashOutput { //LockResult<MutexGuard<MD5HashCashOuput
                            seed: s,
                            hashcode: hashcode.clone()
                        };
                    }
                }
            }))
        }

        for th in threads {
            th.join().unwrap(); //rsult
        }
        let (_, res) = &*mutex_for_result;

        let res = &*res.lock().unwrap(); //LockResult<MutexGuard<MD5HashCashOuput
        MD5HashCashOutput {
            seed: res.seed,
            hashcode: res.hashcode.clone()
        }
    }

    fn verify(&self, answer: MD5HashCashOutput) -> bool {
        let checker : Md5Checker = Md5Checker::new();
        let md5 = answer.hashcode.clone();
        let mut nb_bytes_to0 = 0;
        let chars: Vec<char> = md5.chars().collect();

        chars
            .iter()
            .try_for_each(|letter| {
                let nb_zero: u32 = checker.get_bits_to_zero(letter.to_string()) ; //tester lettre avec checker
                nb_bytes_to0 += nb_zero;

                if nb_zero < 4 {
                    return ControlFlow::Break(letter)
                }
                ControlFlow::Continue(())
            });
        return nb_bytes_to0 >= self.input.complexity;
    }
}


