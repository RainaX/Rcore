impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut max_len: i32 = 0;
        let mut dp = vec![vec![0i32; b.len() + 1]; a.len() + 1];
        for i in (1..a.len() + 1) {
            for j in (1..b.len() + 1) {
                if a[i - 1] == b[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    max_len = std::cmp::max(max_len, dp[i][j]);
                }
            }
        }
        max_len
    }
}
