use std::time::Instant;

pub fn e6() -> (i32, u128){
    let start_time = Instant::now();
    let sum_of_the_squares = 100 * (100 + 1)* 100 * (100 + 1)/4;
    let square_of_the_sum = (100 * (100 + 1) * (2 * 100 + 1)) / 6;
    (sum_of_the_squares - square_of_the_sum, start_time.elapsed().as_nanos())
}