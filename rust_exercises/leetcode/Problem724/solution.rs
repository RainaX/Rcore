impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left_sum: i32 = 0;
        let mut right_sum: i32 = nums.iter().sum();
        
        let mut pivot: usize = 0;
        
        while pivot < nums.len() {
            right_sum -= nums[pivot];
            if left_sum == right_sum {
                return pivot as i32;
            }
            left_sum += nums[pivot];
            pivot += 1;
        }
        
        -1
    }
}
