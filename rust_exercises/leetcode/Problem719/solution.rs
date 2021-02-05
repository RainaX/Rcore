impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        
        let mut lo: i32 = 0;
        let mut hi: i32 = nums[nums.len() - 1] - nums[0];
        while lo < hi {
            let mi = (lo + hi) / 2;
            
            let mut count: i32 = 0;
            let mut left: usize = 0;
            let mut right: usize = 0;
            
            while left < nums.len() {
                while right < nums.len() && nums[right] - nums[left] <= mi {
                    right += 1;
                }
                count += (right - left - 1) as i32;
                left += 1;
            }
            
            if count >= k {
                hi = mi;
            } else {
                lo = mi + 1;
            }
        }
        
        lo
    }
}
