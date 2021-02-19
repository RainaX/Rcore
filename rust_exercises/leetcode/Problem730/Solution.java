class Solution {
    public int countPalindromicSubsequences(String S) {
        
        int[][] prev = new int[S.length()][4];
        int[][] next = new int[S.length()][4];
        
        int mod = 1000000007;
        
        for (int i = 0; i < S.length(); i += 1) {
            for (int k = 0; k < 4; k += 1) {
                prev[i][k] = -1;
                next[i][k] = -1;
            }
        }
        
        for (int i = 0; i < S.length(); i += 1) {
            for (int k = 0; k < 4; k += 1) {
                if (S.charAt(i) == 'a' + (char) k) {
                    prev[i][k] = i;
                } else if (i > 0) {
                    prev[i][k] = prev[i - 1][k];
                }
            }
        }
        
        for (int i = S.length() - 1; i >= 0; i -= 1) {
            for (int k = 0; k < 4; k += 1) {
                if (S.charAt(i) == 'a' + (char) k) {
                    next[i][k] = i;
                } else if (i < S.length() - 1) {
                    next[i][k] = next[i + 1][k];
                }
            }
        }
        
        int[][] dp = new int[S.length()][S.length()];
        for (int l = 1; l <= S.length(); l += 1) {
            for (int i = 0; i + l - 1 < S.length(); i += 1) {
                int j = i + l - 1;
                dp[i][j] = 1;
                
                for (int k = 0; k < 4; k += 1) {
                    int left = next[i][k];
                    int right = prev[j][k];
                    if (left >= i && left <= j) {
                        dp[i][j] += 1;
                        if (left < right) {
                            if (left + 1 == right) {
                                dp[i][j] += 1;
                            } else {
                                dp[i][j] += dp[left + 1][right - 1];
                            }
                        }
                    }
                    dp[i][j] %= mod;
                }
            }
        }
        
        return dp[0][S.length() - 1] - 1;
    }
}
