mod utils;
use utils::file::open_or_create_file;
use utils::perfect_num::{is_mersenne_prime, is_perfect_number};
use utils::prime::sieve_of_eratosthenes;

use std::env;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // コマンドライン引数用の変数の宣言
    let command_type: &str;
    let mut point_number: u32 = 0;

    // コマンドライン引数の数を確認して値を変数に格納
    if args.len() == 1 {
        command_type = &args[1];
    } else if args.len() == 2 {
        command_type = &args[1];
        point_number = match args[2].parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Error: Invalid number provided");
                std::process::exit(1);
            }
        };
    } else {
        eprintln!("Usage: <command> <number>");
        std::process::exit(1);
    }

    match command_type {
        "prime" => prime_search(point_number),
        "perfect_num" => perfect_num_search(point_number),
        _ => println!("Usage: <command> <number>"),
    }

    println!("Program has finished");
}

fn prime_search(point_number: u32) {
    println!("start prime_search");
}

fn perfect_num_search(point_number: u32) {
    println!("start perfect_num_search");
}
