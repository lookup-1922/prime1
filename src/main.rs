mod utils;
use utils::file::open_or_create_file;
use utils::perfect_num::{is_mersenne_prime, is_perfect_number};
use utils::prime::sieve_of_eratosthenes;

use std::env;

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();

    // コマンドライン引数用の変数の宣言
    let mut command_type: String;
    let mut max_number: u32;

    // コマンドライン引数の数を確認して値を変数に格納
    if args.len() == 1 {
        command_type = args[1];
    } else if args.len() == 2 {
        command_type = args[1];
        max_number = match args[2].parse::<u32>() {
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
        prime => prime_search(),
        perfect_num => perfect_num_search(),
        _ => println!("Usage: <command> <number>"),
    }

    let primes = sieve_of_eratosthenes(max_number);
    let mut file = open_or_create_file("prime1.txt");

    for (i, &prime) in primes.iter().enumerate() {
        if i > 0 {
            write!(file, ", ").expect("Failed to write to file");
        }
        write!(file, "{}", prime).expect("Failed to write to file");

        if is_mersenne_prime(prime) {
            println!("{} is a Mersenne prime", prime);
        }

        if is_perfect_number(prime) {
            println!("{} is a perfect number", prime);
        }
    }

    println!(
        "Prime numbers up to {} have been saved to prime1.txt",
        max_number
    );
    println!("Program has finished");
}

fn prime_search() {
    println!("start prime_search");
}

fn perfect_num_search() {
    println!("start perfect_num_search");
}
