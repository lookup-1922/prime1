use num_bigint::BigInt;
use num_traits::{One, Zero, ToPrimitive};
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
    while power < BigInt::from(1000) { // 任意の範囲
        if is_prime(&power) && mersenne_primes_checker(&(BigInt::from(2).pow(power.to_u64().unwrap().try_into().unwrap()) - 1)) {
            println!("2^{} - 1 はメルセンヌ素数です。", power);
        }
        power += 1;
    }
}


/*
    pub fn test_by_number(number: u128) -> bool {
        if is_prime::simple(number) {
            let mersenne_exponent: u32 = (number as f64).log2() as u32;
            let mersenne_number: u128 = 2u128.pow(mersenne_exponent) - 1;

            if mersenne_number == number {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
*/
