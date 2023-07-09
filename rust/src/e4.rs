use std::time::Instant;
//abccba = p * q
// 100000  * a + 10000 * b + 1000 * c + 100 * c + 10 * b + 1 * a = p * q
// 100001 * a + 10010 * b + 1100 * c = p * q
// 11 * (9091 * a + 910 * b + 100 * c) = p * q
pub fn e4() -> (i32, u128) {
    let start_time = Instant::now();
    let mut largest = 0;
    for i in (110..=990).rev().step_by(11){
        for j in (100..=999).rev(){
            let num = i * j;
            if num < largest{
                break
            }
            let d1 = num/(100000)%10;
            let d6 = num%10;
            if d1 != d6{
                continue
            }
            let d2 = num/(10000)%10;
            let d5 = num/(10)%10;
            if d2 != d5{
                continue
            }
            let d3 = num/(1000)%10;
            let d4 = num/(100)%10;
            if d3 != d4{
                continue
            }
            largest = num;
        }
    }
    (largest, start_time.elapsed().as_nanos())
}
