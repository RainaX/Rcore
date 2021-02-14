impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result = vec![];
        
        for x in left..=right {
            let mut temp = x;
            while temp > 0 {
                let digit = temp % 10;
                if digit == 0 || x % digit != 0 {
                    break;
                }
                temp /= 10;
            }
            
            if temp == 0 {
                result.push(x as i32);
            }
        }
        
        result
    }
}
