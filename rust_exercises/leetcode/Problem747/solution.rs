impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        
        let mut id1: i32 = -1;
        let mut id2: i32 = -1;
        for i in 0..nums.len() {
            if id1 == -1 || nums[i] > nums[id1 as usize] {
                id2 = id1;
                id1 = i as i32;
            } else if id2 == -1 || nums[i] > nums[id2 as usize] {
                id2 = i as i32;
            }
        }
        
        if nums[id1 as usize] >= 2 * nums[id2 as usize] {
            id1
        } else {
            -1
        }
    }
}
