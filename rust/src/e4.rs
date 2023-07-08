use std::time::Instant;

pub fn e4() -> (i32, u128) {
    let start_time = Instant::now();
    let mut largest = 0;
    for i in 100..999{
        for j in 100..999{
            let num = i * j;
            let digit1: i32 = num/(100000)%10;
            let digit2 = num/(10000)%10;
            let digit3 = num/(1000)%10;
            let digit4 = num/(100)%10;
            let digit5 = num/(10)%10;
            let digit6 = num%10;
            if digit1 == digit6 && digit2 == digit5 && digit3 == digit4 && num > largest{
                largest = num;
            }
        }
    }
    (largest, (Instant::now() - start_time).as_millis())
}
