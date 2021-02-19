impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let s = s.as_bytes();
        let mut prev = vec![vec![-1; 4]; s.len()];
        let mut next = vec![vec![-1; 4]; s.len()];
        
        const modulo: i32 = 1_000_000_007;
        
        for i in 0..s.len() {
            for k in 0..4 {
                if s[i] == 'a' as u8 + k as u8 {
                    prev[i][k] = i as i32;
                } else {
                    if i > 0 {
                        prev[i][k] = prev[i - 1][k];
                    }
                }
            }
        }
        
        for i in (0..s.len()).rev() {
            for k in 0..4 {
                if s[i] == 'a' as u8 + k as u8 {
                    next[i][k] = i as i32;
                } else {
                    if i < s.len() - 1 {
                        next[i][k] = next[i + 1][k];
                    }
                }
            }
        }
        
        let mut dp: Vec<Vec<i32>> = vec![vec![1; s.len()]; s.len()];
        for l in 1..=s.len() {
            for i in 0..=(s.len() - l) {
                let j = i + l - 1;
                
                for k in 0..4 {
                    let left = next[i][k];
                    let right = prev[j][k];
                    
                    if left >= i as i32 && left <= j as i32 {
                        dp[i][j] += 1;
                        
                        if left < right {
                            if left + 1 == right {
                                dp[i][j] += 1;
                            } else {
                                dp[i][j] += dp[left as usize + 1][right as usize - 1];
                            }
                        }
                    }
                    
                    dp[i][j] %= modulo;
                }
            }
        }
        
        dp[0][s.len() - 1] - 1
    }
}
