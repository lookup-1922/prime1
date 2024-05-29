use num_bigint::{BigInt, BigUint, ToBigUint};
use num_primes::Verification;
use num_traits::{One, ToPrimitive, Zero};

pub fn is_composite(number: &BigInt) -> bool {
    !is_prime(number)
}

pub fn is_prime(number: &BigInt) -> bool {
    let number: BigUint = match number.to_biguint() {
        Some(n) => n,
        None => return false,
    };
    Verification::is_prime(&number)
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
    return false;
}

pub fn perfect_number_checker(number: &BigInt, divisor: &Vec<BigInt>) -> bool {
    let sum_divisors: BigInt = divisor.iter().sum();
    if number * 2 == sum_divisors {
        return true;
    } else {
        return false;
    }
}

pub fn mersenne_to_perfect(power: &BigInt) -> BigInt {
    BigInt::from(2).pow(power.to_u64().unwrap() - 1)
        * (BigInt::from(2).pow(power.to_u64().unwrap()) - 1)
}

pub fn mersenne_search() {
    let mut power = BigInt::from(2);
    while power < BigInt::from(1000) {
        // 任意の範囲
        if is_prime(&power)
            && mersenne_primes_checker(&(BigInt::from(2).pow(power.to_u64().unwrap()) - 1))
        {
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
