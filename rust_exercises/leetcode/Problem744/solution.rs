impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut lo = 0;
        let mut hi = letters.len();
        
        while lo < hi {
            let mi = lo + (hi - lo) / 2;
            if (letters[mi] <= target) {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        
        letters[lo % letters.len()]
    }
}
