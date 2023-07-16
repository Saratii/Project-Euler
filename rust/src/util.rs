pub struct Big{
    pub digits: Vec<u32>
}
impl Big {
    pub fn new(numby: &str) -> Self{
        Big { digits: (numby.chars().filter_map(|c| c.to_digit(10)).collect()) }
    }
    pub fn add(mut self, mut b: Big) -> Big{
        while self.digits.len() > b.digits.len(){
            b.digits.insert(0, 0);
        }
        while self.digits.len() < b.digits.len(){
            self.digits.insert(0, 0);
        }
        for i in (0..self.digits.len()).rev() {
            self.digits[i] += b.digits[i];
            if self.digits[i] > 9{
                if i == 0{
                    self.digits.insert(0, 1);
                    self.digits[i+1] %= 10;
                    continue
                }
                self.digits[i-1] += 1;
                self.digits[i] %= 10;
            }
        }
        self
    }
}

pub fn combinatorics(total_size: usize, options: usize) -> usize{
    factorial(total_size) / (factorial(options) * factorial(total_size - options))
}

pub fn factorial(n: usize) -> usize{
    (2..=n).product()
}