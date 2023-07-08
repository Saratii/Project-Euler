use crate::e1::e1;
use crate::e2::e2;
use crate::e3::e3;
use crate::e4::e4;

pub mod e1;
mod e2;
mod e3;
mod e4;

fn main() {
    let (value, exec_time) = e1();
    println!("problem 1 returned {} in {} ms", value, exec_time);
    let (value, exec_time) = e2();
    println!("problem 2 returned {} in {} ms", value, exec_time);
    let (value, exec_time) = e3();
    println!("problem 3 returned {} in {} ms", value, exec_time);
    let (value, exec_time) = e4();
    println!("problem 4 returned {} in {} ms", value, exec_time);
}
