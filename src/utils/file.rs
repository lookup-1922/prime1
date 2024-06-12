use dialoguer::FuzzySelect;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::fs::File;
use std::io::{self, BufRead};
use walkdir::WalkDir;

pub fn select_file(dir: &str, file_extension: &str) -> str{
    let mut options = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == file_extension {
                    options.push(path.to_string_lossy().into_owned());
                }
            }
        }
    }

    if options.len() != 0 {
        let filename = FuzzySelect::new()
            .with_prompt("ファイルを選択してください")
            .items(&options)
            .interact()
            .unwrap();

        let filename = &options[filename];
        return filename;
    } else {
        return "123";
    }
}

pub fn file_convert(filename: &str) {
    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let number: BigInt = line.trim().parse().unwrap();
        if number.is_even() {
            let power = number.log2() + 1;
            if crate::utils::prime::is_prime(&BigInt::from(power))
                && crate::utils::prime::mersenne_primes_checker(&number)
            {
                let perfect = crate::utils::prime::mersenne_to_perfect(&BigInt::from(power));
                println!("完全数: {}", perfect);
            }
        }
    }
}

pub fn find_txt_files(dir: &str) -> Vec<String> {
    let mut options = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "txt" {
                    options.push(path.to_string_lossy().into_owned());
                }
            }
        }
    }
    options
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
