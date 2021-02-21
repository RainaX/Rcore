use std::collections::BTreeMap;
use std::cmp::max;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut map = BTreeMap::new();
        for num in nums.into_iter() {
            if let Some(x) = map.get_mut(&num) {
                *x = *x + num;
            } else {
                map.insert(num, num);
            }
        }
        
        let mut sum0 = 0;
        let mut sum1 = 0;
        let mut prev = 0;
        
        for (i, (&k, &v)) in map.iter().enumerate() {
            if (i > 0 && k == prev + 1) {
                let temp = sum0;
                sum0 = max(sum0, sum1);
                sum1 = temp + v;
            } else {
                sum0 = max(sum0, sum1);
                sum1 = sum0 + v;
            }
            prev = k;
        }
        
        max(sum0, sum1)
    }
}
