impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut n = n;
        let mut digits = vec![];
        
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        
        let mut lo = 0;
        let mut hi = 1;
        
        while hi < digits.len() {
            if digits[hi] > digits[hi - 1] {
                digits[hi] -= 1;
                for i in lo..hi {
                    digits[i] = 9;
                }
                lo = hi;
            }
            hi += 1;
        }
        
        let mut result = 0;
        while let Some(d) = digits.pop() {
            result = result * 10 + d;
        }
        
        result
    }
}
