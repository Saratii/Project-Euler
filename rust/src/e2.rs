use std::time::Instant;

pub fn e2() -> (i32, u128){
    let start_time = Instant::now();
    let mut sum = 0;
    let (mut a, mut b, mut c) = (1, 1, 0);
    while c <= 4_000_000{
        c = a + b;
        if c % 2 == 0{
            sum += c;
        }
        a = b;
        b = c;
    }   
    (sum, (Instant::now() - start_time).as_nanos())
}