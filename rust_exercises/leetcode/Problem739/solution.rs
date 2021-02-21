impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut result = vec![0; t.len()];
        
        for (i, temp) in t.iter().enumerate() {
            while !stack.is_empty() {
                let (index, value) = stack[stack.len() - 1];
                if temp <= value {
                    break;
                }
                result[index] = (i - index) as i32;
                stack.pop();
            }
            stack.push((i, temp));
        }
        
        result
    }
}
