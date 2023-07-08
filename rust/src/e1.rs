use std::time::Instant;

pub fn e1() -> (i32, u128){
    let start_time = Instant::now();
    let mut sum = 0;
    for i in 0..1000{
        if i % 5 == 0 || i % 3 == 0{
            sum += i;
        }
    }
    (sum, (Instant::now() - start_time).as_millis())
}