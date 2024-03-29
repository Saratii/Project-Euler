use std::time::Instant;

pub fn e3() -> (i32, u128) {
    let start_time = Instant::now();
    let mut target: f64 = 600851475143.0_f64;
    while target % 2.0 == 0.0 {
        target /= 2.0;
    }
    let mut i = 3.0;
    while i * i <= target {
        while target % i == 0.0 {
            target /= i;
        }
        i += 2.0;
    }
    (if target < 2.0 { 2 } else { target as i32 }, start_time.elapsed().as_nanos())
}
