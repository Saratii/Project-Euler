use std::time::Instant;

const PRIMES: [i32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

pub fn e5() -> (i32, u128) {
    let start_time = Instant::now();
    let max = 20;
    let mut max_num_primes = [1; PRIMES.len()];
    for i in 2..=max{
        let prime_factorization = prime_factorization(i);
        for j in 0..max_num_primes.len(){
            if prime_factorization[j] > max_num_primes[j]{
                max_num_primes[j] = prime_factorization[j];
            }
        }
    }
    let mut target = 1;
    for i in 0..PRIMES.len(){
        target *= PRIMES[i as usize].pow(max_num_primes[i as usize] as u32);
    }
    (target, start_time.elapsed().as_nanos())

}
fn prime_factorization(mut n: i32) -> [i32; 8]{
    let mut result = [0; PRIMES.len()];
    for i in 0..PRIMES.len(){
        while n % PRIMES[i] == 0{
            result[i] += 1;
            n /= PRIMES[i];
        }
    }
    result
}