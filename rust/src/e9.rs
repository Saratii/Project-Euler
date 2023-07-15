use std::time::Instant;

pub fn e9() -> (i32, u128){
    let start_time = Instant::now();
    for a in 0..1000{
        for b in 0..1000{
            for c in 0..1000{
                if a >= b || b >= c {
                    continue
                }
                if a*a + b*b == c*c{
                    if a + b + c == 1000{
            
                        return (a * b * c, start_time.elapsed().as_nanos())
                    }
                }
            }
        }
    }
    (0, start_time.elapsed().as_nanos())
}