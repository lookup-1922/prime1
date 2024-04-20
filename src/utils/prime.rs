// エラトステネスの篩を用いて素数を見つける関数
pub fn sieve_of_eratosthenes(max_num: u32) -> Vec<u32> {
    let mut is_prime = vec![true; (max_num + 1) as usize];
    let mut primes = Vec::new();

    for num in 2..=max_num {
        if is_prime[num as usize] {
            primes.push(num);
            let mut multiple = num * num;
            while multiple <= max_num {
                is_prime[multiple as usize] = false;
                multiple += num;
            }
        }
    }

    primes
}
