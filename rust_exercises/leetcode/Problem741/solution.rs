use std::cmp::max;
use std::cmp::min;


impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        
        let mut dp = vec![vec![vec![-1; n]; n]; 2 * n - 1];
        dp[0][0][0] = grid[0][0];
        
        for step in 1..(2 * n as i32 - 1) {
            for i1 in max(0, step - n as i32 + 1)..=min(n as i32 - 1, step as i32) {
                for i2 in max(0, step - n as i32 + 1)..=min(n as i32 - 1, step as i32) {
                    let step = step as usize;
                    let i1 = i1 as usize;
                    let i2 = i2 as usize;
                    let j1 = step - i1;
                    let j2 = step - i2;
                    
                    if grid[i1][j1] >= 0 && grid[i2][j2] >= 0 {
                        let base = if i1 == i2 {
                            grid[i1][j1]
                        } else {
                            grid[i1][j1] + grid[i2][j2]
                        };
                        
                        if i1 > 0 && i2 > 0 && dp[step - 1][i1 - 1][i2 - 1] >= 0 {
                            dp[step][i1][i2] = max(dp[step][i1][i2], base + dp[step - 1][i1 - 1][i2 - 1]);
                        }
                        
                        if i1 > 0 && j2 > 0 && dp[step - 1][i1 - 1][i2] >= 0 {
                            dp[step][i1][i2] = max(dp[step][i1][i2], base + dp[step - 1][i1 - 1][i2]);
                        }
                        
                        if j1 > 0 && i2 > 0 && dp[step - 1][i1][i2 - 1] >= 0 {
                            dp[step][i1][i2] = max(dp[step][i1][i2], base + dp[step - 1][i1][i2 - 1])
                        }
                        
                        if j1 > 0 && j2 > 0 && dp[step - 1][i1][i2] >= 0 {
                            dp[step][i1][i2] = max(dp[step][i1][i2], base + dp[step - 1][i1][i2]);
                        }
                    }
                }
            }
        }
        
        max(0, dp[2 * n - 2][n - 1][n - 1])
    }
}
