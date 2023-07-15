use std::time::Instant;

pub fn e10() -> (i64, u128){
    let start_time = Instant::now();
    let cap = 2_000_000 - 1;
    
    let primes = sieve(cap);
    
    (primes.iter().sum(), start_time.elapsed().as_nanos())
}

fn sieve(cap: usize) -> Vec<i64>{
    let mut prime_sieve: Vec<i64> = vec![0; cap + 1];
    for i in 0..cap + 1{
        prime_sieve[i] = i as i64;
    }
    prime_sieve[1] = 0;
    let mut i = 2;
    while i*i <= cap{
        if prime_sieve[i] != 0{
            for j in (i * i..=cap).step_by(i){
                prime_sieve[j] = 0;
            }
        }
        i += 1;
    }
    return prime_sieve;
}