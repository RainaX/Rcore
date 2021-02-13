class Solution {
    public String minWindow(String S, String T) {
        int[][] dp = new int[S.length() + 1][T.length() + 1];
        int[][] start = new int[S.length() + 1][T.length() + 1];
        String result = null;
        
        for (int i = 1; i <= S.length(); i += 1) {
            for (int j = 1; j <= T.length(); j += 1) {
                if (S.charAt(i - 1) == T.charAt(j - 1)) {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    if (dp[i][j] > 1) {
                        start[i][j] = start[i - 1][j - 1];
                    } else {
                        start[i][j] = i;
                    }
                } else {
                    if (dp[i - 1][j] > dp[i][j - 1]) {
                        dp[i][j] = dp[i - 1][j];
                        start[i][j] = start[i - 1][j];
                    } else if (dp[i - 1][j] < dp[i][j - 1]) {
                        dp[i][j] = dp[i][j - 1];
                        start[i][j] = start[i][j - 1];
                    } else {
                        dp[i][j] = dp[i - 1][j];
                        start[i][j] = Math.max(start[i - 1][j], start[i][j - 1]);
                    }
                }
                
                if (dp[i][j] == T.length()) {
                    if (result == null || i - start[i][j] + 1 < result.length()) {
                        result = S.substring(start[i][j] - 1, i);
                    }
                }
            }
        }
        
        if (result == null) {
            return "";
        }
        return result;
    }
}
