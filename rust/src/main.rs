use crate::e1::e1;
use crate::e2::e2;
use crate::e3::e3;
use crate::e4::e4;
use crate::e5::e5;
use crate::e6::e6;
use crate::e7::e7;
use crate::e8::e8;
use crate::e9::e9;

mod e1;
mod e2;
mod e3;
mod e4;
mod e5;
mod e6;
mod e7;
mod e8;
mod e9;

fn main() {
    let (value, exec_time) = e1();
    println!("problem 1 returned {} in {} ns", value, exec_time);
    let (value, exec_time) = e2();
    println!("problem 2 returned {} in {} ns", value, exec_time);
    let (value, exec_time) = e3();
    println!("problem 3 returned {} in {} ns", value, exec_time);
    let (value, exec_time) = e4();
    println!("problem 4 returned {} in {} ns", value, exec_time);
    let (value, exec_time) = e5();
    println!("problem 5 returned {} in {} ns", value, exec_time);
    let (value, exec_time) = e6();
    println!("problem 6 returned {} in {} ns", value, exec_time);
    let (value, exec_time) = e7();
    println!("problem 7 returned {} in {} ns", value, exec_time);
    let (value, exec_time) = e8();
    println!("problem 8 returned {} in {} ns", value, exec_time);
    let (value, exec_time) = e9();
    println!("problem 9 returned {} in {} ns", value, exec_time);
}
