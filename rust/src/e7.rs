use std::time::Instant;

pub fn e7() -> (i32, u128){
    let start_time = Instant::now();
    let mut prime_count = 1;
    let mut i = 3;
    loop {
        if is_prime(i){
            prime_count += 1;
            if prime_count == 10001{
                break
            }
        }
        i += 2;
    }
    (i, start_time.elapsed().as_nanos())
}

fn is_prime(n: i32) -> bool{
    if n == 2{
        return true
    }
    let mut i = 3;
    while i * i <= n{
        if n % i == 0{
            return false
        }
        i += 2;
    }
    return true
}