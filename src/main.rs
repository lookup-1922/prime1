mod utils;

use dialoguer::{theme::ColorfulTheme, MultiSelect};
use num_bigint::BigInt;
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("任意の自然数を入力してください。");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let number: BigInt = input.trim().parse().unwrap();

        let options = &[
            "約数",
            "合成数判定",
            "素数判定",
            "メルセンヌ素数判定",
            "完全数判定",
        ];
        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("何を実行しますか？")
            .items(options)
            .interact()
            .unwrap();

        if selections.contains(&0) {
            let divisor = utils::file::find_divisor(&number);
            println!("約数: {:?}", divisor);
        }
        if selections.contains(&1) {
            if !utils::prime::is_composite(&number) {
                println!("合成数ではありません。");
            } else {
                println!("合成数です。");
            }
        }
        if selections.contains(&2) {
            if utils::prime::is_prime(&number) {
                println!("素数です。");
            } else {
                println!("素数ではありません。");
            }
        }
        if selections.contains(&3) {
            if utils::prime::mersenne_primes_checker(&number) {
                println!("メルセンヌ素数です。");
            } else {
                println!("メルセンヌ素数ではありません。");
            }
        }
        if selections.contains(&4) {
            let divisor = utils::file::find_divisor(&number);
            if utils::prime::perfect_number_checker(&number, &divisor) {
                println!("完全数です。");
            } else {
                println!("完全数ではありません。");
            }
        }
    } else if args.len() == 2 && args[1] == "--mersenne-search" {
        utils::prime::mersenne_search();
    } else if args.len() == 3 && args[1] == "--file-convert" {
        let filename = &args[2];
        utils::file::file_convert(filename);
    }
}
