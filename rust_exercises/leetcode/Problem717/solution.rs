impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut count: u32 = 0;
        for bit in bits.iter().rev().skip(1) {
            if *bit == 0 {
                break;
            }
            count += 1;
        }
        count % 2 == 0
    }
}
