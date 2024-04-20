mod utils;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <max_number>", args[0]);
        std::process::exit(1);
    }

    let max_number = match args[1].parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number provided");
            std::process::exit(1);
        }
    };

    let primes = utils::prime::sieve_of_eratosthenes(max_number);
    let mut file = utils::file::open_or_create_file("primes.txt");

    for (i, &prime) in primes.iter().enumerate() {
        if i > 0 {
            write!(file, ", ").expect("Failed to write to file");
        }
        write!(file, "{}", prime).expect("Failed to write to file");

        if utils::perfect_num::is_mersenne_prime(prime) {
            println!("{} is a Mersenne prime", prime);
        }

        if utils::perfect_num::is_perfect_number(prime) {
            println!("{} is a perfect number", prime);
        }
    }

    println!(
        "Prime numbers up to {} have been saved to primes.txt",
        max_number
    );
}
