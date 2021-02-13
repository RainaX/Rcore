impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut dp = vec![vec![0 as usize; t.len() + 1]; s.len() + 1];
        let mut start = vec![vec![0 as usize; t.len() + 1]; s.len() + 1];
        let mut result: Option<&[u8]> = None;
        
        for i in 1..s.len() + 1 {
            for j in 1..t.len() + 1 {
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    if dp[i][j] > 1 {
                        start[i][j] = start[i - 1][j - 1];
                    } else {
                        start[i][j] = i;
                    }
                } else {
                    if dp[i - 1][j] > dp[i][j - 1] {
                        dp[i][j] = dp[i - 1][j];
                        start[i][j] = start[i - 1][j];
                    } else if dp[i - 1][j] < dp[i][j - 1] {
                        dp[i][j] = dp[i][j - 1];
                        start[i][j] = start[i][j - 1];
                    } else {
                        dp[i][j] = dp[i - 1][j];
                        start[i][j] = std::cmp::max(start[i - 1][j], start[i][j - 1]);
                    }
                }
                
                if dp[i][j] == t.len() {
                    result = match result {
                        Some(r) => {
                            if r.len() > i - start[i][j] + 1 {
                                Some(&s[start[i][j] - 1..i])
                            } else {
                                Some(r)
                            }
                        },
                        None => Some(&s[start[i][j] - 1..i]),
                    };
                }
            }
        }
        
        match result {
            Some(r) => r.iter().map(|x| *x as char).collect(),
            None => String::new(),
        }
    }
}
