use std::time::Instant;

pub fn e14() -> (usize, u128){
    let start_time = Instant::now();
    let mut max = 0;
    let mut max_index = 0;
    let mut memo: Vec<usize> = vec![0; 1_000_000];

    for n in (1..1_000_000).step_by(2){
        let mut num_terms = 1;
        let mut nn = n as usize;
        while nn != 1{
            if nn < 1_000_000 && memo[nn] != 0 {
                num_terms += memo[nn] - 1;
                break;
            }
            if nn % 2 == 0{
                nn /= 2;
            } else {
                nn = nn * 3 + 1;
            }
            num_terms += 1;
        }
        memo[n] = num_terms;

        if num_terms > max{
            max = num_terms;
            max_index = n;
        }
    }
    (max_index, start_time.elapsed().as_nanos())
}
