use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;
use md5::compute;
use serde::{Deserialize, Serialize};

use crate::challenge::IChallenge;

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

fn is_valid_hash(hex: String, expected_dif: u32) -> bool {
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
        let nb_thread: u64 = 16;
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

                    let is_valid = is_valid_hash(
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
        let md5 = answer.hashcode.clone();

        is_valid_hash(md5, self.input.complexity)
    }
}

#[test]
fn test_md5hash_cash_solve(){
    let md5hash_cash_input: MD5HashCashInput = MD5HashCashInput{
        complexity: 9,
        message: String::from("hello")
    };
    let md5hash_cash: MD5HashCash = MD5HashCash::new(md5hash_cash_input);
    let md5hash_cash_output: MD5HashCashOutput = md5hash_cash.solve();
    let result : bool = md5hash_cash.verify(md5hash_cash_output);
    assert_eq!(result, true);
}

#[test]
fn test_md5hash_cash_verify(){
    let md5_hash_cash_input: MD5HashCashInput = MD5HashCashInput{
        complexity: 9,
        message: String::from("hello")
    };
    let md5_hash_cash = MD5HashCash::new(md5_hash_cash_input);
    let result = md5_hash_cash.verify(MD5HashCashOutput {
        seed: 844,
        hashcode: String::from("00441745D9BDF8E5D3C7872AC9DBB2C3")
    });
    assert_eq!(result, true);
}

#[test]
fn test_is_valid_hash(){
    let result = is_valid_hash(String::from("00441745D9BDF8E5D3C7872AC9DBB2C3"), 9);
    assert_eq!(result, true);
}

