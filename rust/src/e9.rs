use std::time::Instant;

pub fn e9() -> (i32, u128){
    let start_time = Instant::now();
    for a in 0..1000 / 3 - 2{
        for b in a + 1..=(1000 - a) / 2{
            let c = 1000 - a - b;
            if c <= b {
                continue
            }
            if a*a + b*b == c*c{
                return (a * b * c, start_time.elapsed().as_nanos())
            }
        }
    }
    (0, start_time.elapsed().as_nanos())
}