use indicatif::{ProgressBar, ProgressStyle};
use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive};
use std::fs::File;
use std::io::{self, BufRead};

pub fn find_divisor(number: &BigInt) -> Vec<BigInt> {
    let mut result = Vec::new();
    let mut i = BigInt::one();

    let amount = number.sqrt();
    let bar = ProgressBar::new(amount.to_u64().unwrap());
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("#>-")
    );
    bar.set_message("checking divisor");

    while &i * &i <= *number {
        if number % &i == BigInt::zero() {
            result.push(i.clone());
            if &i * &i != *number {
                result.push(number / &i);
            };
        }
        i += 1;
        bar.set_position(i.to_u64().unwrap());
    }

    bar.finish_with_message("Done!");

    result.sort();
    return result;
}

pub fn file_convert(filename: &str) {
    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let number: BigInt = line.trim().parse().unwrap();
        if number.is_even() {
            let power = number.log2() + 1;
            if crate::utils::prime::is_prime(&BigInt::from(power)) && crate::utils::prime::mersenne_primes_checker(&number) {
                let perfect = crate::utils::prime::mersenne_to_perfect(&BigInt::from(power));
                println!("完全数: {}", perfect);
            }
        }
    }
}

trait BigIntExt {
    fn is_even(&self) -> bool;
    fn log2(&self) -> u64;
}

impl BigIntExt for BigInt {
    fn is_even(&self) -> bool {
        self % 2 == BigInt::zero()
    }

    fn log2(&self) -> u64 {
        let mut n = self.clone();
        let mut count = 0;
        while n > BigInt::one() {
            n /= 2;
            count += 1;
        }
        count
    }
}
