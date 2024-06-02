use indicatif::{ProgressBar, ProgressStyle};
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive, Zero};
use std::convert::TryInto;

pub fn is_composite(number: &BigInt) -> bool {
    !is_prime(number)
}

pub fn is_prime(number: &BigInt) -> bool {
    if number <= &BigInt::one() {
        return false;
    }

    let mut i = BigInt::from(2);
    while &i * &i <= *number {
        if number % &i == BigInt::zero() {
            return false;
        }
        i += 1;
    }
    true
}

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

pub fn mersenne_primes_checker(number: &BigInt) -> bool {
    let one = BigInt::one();
    if (number + &one & number) == BigInt::zero() {
        let mut i = BigInt::one();
        while &i * &i <= *number {
            if number % &i == BigInt::zero() {
                return false;
            }
            i += 1;
        }
        return true;
    }
    false
}

pub fn perfect_number_checker(number: &BigInt, divisor: &Vec<BigInt>) -> bool {
    let sum_divisors: BigInt = divisor.iter().sum();
    number * 2 == sum_divisors
}

pub fn mersenne_to_perfect(power: &BigInt) -> BigInt {
    let power_u32: u32 = (power.to_u64().unwrap()).try_into().unwrap();
    BigInt::from(2).pow(power_u32 - 1) * (BigInt::from(2).pow(power_u32) - 1)
}

pub fn mersenne_search() {
    let mut power = BigInt::from(2);
    while power < BigInt::from(1000) {
        // 任意の範囲
        if is_prime(&power)
            && mersenne_primes_checker(
                &(BigInt::from(2).pow(power.to_u64().unwrap().try_into().unwrap()) - 1),
            )
        {
            println!("2^{} - 1 はメルセンヌ素数です。", power);
        }
        power += 1;
    }
}

// 2024年での最大のメルセンヌ素数はM82589933
// u64の最大値は18446744073709551615
// テストをパスしていない。
pub fn lucas_lehmer_test(p: usize) -> bool {
    let m = BigInt::from(1) << p;
    let mut s = BigInt::from(4);

    let pb = ProgressBar::new(p as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("##-"),
    );

    for _n in 2..=p {
        let s_squared = &s * &s;
        s = (s_squared - 2) % &m;
        pb.inc(1);
    }

    pb.finish_and_clear();
    s == BigInt::zero()
}
