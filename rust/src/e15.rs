use std::time::Instant;
const N: usize = 20;
pub fn e15() -> (usize, u128){
    let start_time = Instant::now();
    let mut grid = [[1; N + 1]; N + 1];
    for i in 1..N + 1{
        for j in 1..N + 1{
            grid[i][j] = grid[i][j-1] + grid[i-1][j];
        }
    }
    (grid[N][N], start_time.elapsed().as_nanos())
}