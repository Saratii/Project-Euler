use std::time::Instant;

pub fn e12() -> (usize, u128){
    let start_time = Instant::now();  
    let mut num_factors;
    let mut n: usize = 1;
    let mut i = 2;
    let mut primes: Vec<usize> = vec![2, 3];
    loop {
        seef(n, &mut primes);
        let prime_factors = prime_factorization(n, &primes);
        num_factors = count_factors(&prime_factors);
        if num_factors > 500{
            println!("{}", num_factors);
            return (n, start_time.elapsed().as_nanos())
        }
        n += i;
        i += 1;
    }
}

fn prime_factorization(mut n: usize, primes: &Vec<usize>) -> Vec<usize> {
    let mut result = vec![0; primes.len()];
    for i in 0..primes.len() {
        while n % primes[i] == 0 {
            result[i] += 1;
            n /= primes[i];
        }
    }
    result
}

fn seef(cap: usize, primes: &mut Vec<usize>){
    for i in (primes[primes.len() - 1]..(cap as f64).sqrt() as usize).step_by(2){
        let mut j = 0;
        let mut prime = true;
        while j < primes.len(){
            if i % primes[j] == 0{
                prime = false;
                break
            }
            if primes[j] * primes[j] > i{
                break
            }
            j += 1;
        }
        if prime{
            primes.push(i);
        }
    }
}

fn count_factors(prime_factorization: &Vec<usize>) -> usize{
    let mut num_factors = 1;
    for i in prime_factorization{
        if *i != 0{
            num_factors *= i + 1;
        }
    }
    num_factors
}