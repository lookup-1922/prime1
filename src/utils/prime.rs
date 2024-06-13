use indicatif::{ProgressBar, ProgressStyle};
use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, ToPrimitive, Zero};
use num_integer::Integer;
use rand::thread_rng;
use std::convert::TryInto;

// 合成数のときfalse
// 素数かもしれないときtrue
pub fn miller_rabin(n: &BigInt, k: usize) -> bool {
    // If n is 2, it's prime
    if *n == BigInt::from(2) {
        return true;
    }

    // If n is less than 2 or even, it's composite
    if *n < BigInt::from(2) || n.is_even() {
        return false;
    }

    // Write n-1 as 2^s * d
    let one = BigInt::one();
    let two = BigInt::from(2);
    let mut d = n - &one;
    let mut s = 0;

    while &d % &two == BigInt::zero() {
        d /= 2;
        s += 1;
    }

    let mut rng = thread_rng();
    let bar = ProgressBar::new(k as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("##-"),
    );

    for _ in 0..k {
        let a = rng.gen_bigint_range(&two, &(n - &one));
        let mut x = mod_exp(&a, &d, n);

        if x == one || x == n - &one {
            bar.inc(1);
            continue;
        }

        let mut is_composite = true;
        for _ in 0..s - 1 {
            x = mod_exp(&x, &two, n);
            if x == n - &one {
                is_composite = false;
                break;
            }
        }

        if is_composite {
            bar.finish_and_clear();
            return false;
        }
        bar.inc(1);
    }

    bar.finish_and_clear();
    true
}

// base^exp % modulus を計算する関数
fn mod_exp(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
    let mut base = base % modulus;
    let mut exp = exp.clone();
    let mut result = BigInt::one();

    while exp > BigInt::zero() {
        if &exp % 2 == BigInt::one() {
            result = (&result * &base) % modulus;
        }
        exp >>= 1;
        base = (&base * &base) % modulus;
    }

    result
}

pub fn is_prime(number: &BigInt) -> bool {
    if number <= &BigInt::one() {
        return false;
    }

    let mut i = BigInt::from(2);

    let amount = number.sqrt();
    let bar = ProgressBar::new(amount.to_u64().unwrap());
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("#>-")
    );
    bar.set_message("checking");

    while &i * &i <= *number {
        if number % &i == BigInt::zero() {
            return false;
        }
        i += 1;
        bar.set_position(i.to_u64().unwrap());
    }
    bar.finish_and_clear();
    return true;
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

    bar.finish_and_clear();

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
pub fn lucas_lehmer_test(p: usize) -> bool {
    let m = (BigInt::one() << p) - BigInt::one();
    let mut s: BigInt = "4".parse().unwrap();

    let bar = ProgressBar::new(p as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("##-"),
    );
    bar.set_message("lucas_lehmer_test");

    for _n in 2..=p {
        s = (&s * &s - 2) % &m;
        if s == BigInt::zero() {
            return true;
        }
        bar.inc(1);
    }

    bar.finish_and_clear();
    return s == BigInt::zero();
}
