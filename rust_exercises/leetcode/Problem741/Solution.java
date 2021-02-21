class Solution {
    public int cherryPickup(int[][] grid) {
        int n = grid.length;
        
        int[][][] dp = new int[2 * n - 1][n][n];
        
        dp[0][0][0] = grid[0][0];
        for (int step = 1; step < 2 * n - 1; step += 1) {
            for (int i1 = 0; i1 <= step; i1 += 1) {
                for (int i2 = 0; i2 <= step; i2 += 1) {
                    int j1 = step - i1;
                    int j2 = step - i2;
                    
                    if (i1 >= n || i2 >= n || j1 >= n || j2 >= n) {
                        continue;
                    }
                    
                    dp[step][i1][i2] = -1;
                    if (grid[i1][j1] >= 0 && grid[i2][j2] >= 0) {
                        int base;
                        if (i1 == i2) {
                            base = grid[i1][j1];
                        } else {
                            base = grid[i1][j1] + grid[i2][j2];
                        }
                        
                        if (i1 > 0 && i2 > 0 && dp[step - 1][i1 - 1][i2 - 1] >= 0) {
                            dp[step][i1][i2] = Math.max(dp[step][i1][i2], base + dp[step - 1][i1 - 1][i2 - 1]);
                        }
                        
                        if (i1 > 0 && j2 > 0 && dp[step - 1][i1 - 1][i2] >= 0) {
                            dp[step][i1][i2] = Math.max(dp[step][i1][i2], base + dp[step - 1][i1 - 1][i2]);
                        }
                        
                        if (j1 > 0 && i2 > 0 && dp[step - 1][i1][i2 - 1] >= 0) {
                            dp[step][i1][i2] = Math.max(dp[step][i1][i2], base + dp[step - 1][i1][i2 - 1]);
                        }
                        
                        if (j1 > 0 && j2 > 0 && dp[step - 1][i1][i2] >= 0) {
                            dp[step][i1][i2] = Math.max(dp[step][i1][i2], base + dp[step - 1][i1][i2]);
                        }
                    }
                }
            }
        }
        
        return Math.max(0, dp[2 * n - 2][n - 1][n - 1]);
    }
}
