use std::time::Instant;

pub fn e10() -> (i64, u128){
    let start_time = Instant::now();
    let mut sum: i64 = 2;
    for i in (3..2_000_000).step_by(2){
        if is_prime(i){
            sum += i as i64;
        }
    }
    (sum, start_time.elapsed().as_nanos())
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