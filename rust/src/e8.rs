use std::time::Instant;

const INPUT: &str =
"73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450";
    
pub fn e8() -> (u64, u128) {
    let start_time = Instant::now();
    let input_string = INPUT.replace("\n", "").replace(" ", "");
    let numbers: [u64; 1000] = {
        let mut arr: [u64; 1000] = [0; 1000];
        let mut index = 0;
        for c in input_string.chars() {
            if let Some(digit) = c.to_digit(10) {
                arr[index] = digit as u64;
                index += 1;
            }
        }
        arr
    };

    let mut max = 0;
    let mut running_prod = 1;
    let mut i = 0;
    while i + 13 < 1000 {
        if i == 0 {
            for j in 0..13 {
                running_prod *= numbers[j];
            }
        } else {
            if numbers[i - 1] == 0 {
                running_prod = 1;
                for j in 0..13 {
                    if numbers[j] == 0{
                        running_prod = 0;
                    }
                    running_prod *= numbers[i + j];
                }
            } else {
                running_prod *= numbers[i + 12];
                running_prod /= numbers[i - 1];
            }
        }
        if running_prod > max {
            max = running_prod;
        }
        i += 1;
    }
    (max, start_time.elapsed().as_nanos())
}