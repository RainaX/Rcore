class Solution {
    public int containVirus(int[][] grid) {
        int m = grid.length;
        int n = grid[0].length;

        int countWalls = 0;
        while (true) {
            boolean[][] visited = new boolean[m][n];
            Set<Integer> maxThreat = new HashSet<>();
            Set<Integer> maxBlock = new HashSet<>();
            Set<Integer> toInfect = new HashSet<>();
            int maxWalls = 0;
            
            for (int i = 0; i < m; i += 1) {
                for (int j = 0; j < n; j += 1) {
                    if (grid[i][j] == 1 && !visited[i][j]) {
                        Set<Integer> threat = new HashSet<>();
                        Set<Integer> block = new HashSet<>();
                        int walls = dfs(i, j, grid, visited, threat, block);
                        
                        if (threat.size() > maxThreat.size()) {
                            toInfect.addAll(maxThreat);
                            maxThreat = threat;
                            maxBlock = block;
                            maxWalls = walls;
                        } else {
                            toInfect.addAll(threat);
                        }
                    }
                }
            }
            
            if (maxThreat.isEmpty()) {
                break;
            }
            
            for (int pos: toInfect) {
                grid[pos / n][pos % n] = 1;
            }
            
            for (int pos: maxBlock) {
                grid[pos / n][pos % n] = 2;
            }
            
            countWalls += maxWalls;
        }
        
        return countWalls;
    }
    
    private int dfs(int i, int j, int[][] grid, boolean[][] visited, Set<Integer> threat, Set<Integer> block) {
        int m = grid.length;
        int n = grid[0].length;
        
        visited[i][j] = true;
        block.add(i * n + j);
        
        int[][] moves = new int[][]{{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
        int walls = 0;
        
        for (int k = 0; k < 4; k += 1) {
            int ii = i + moves[k][0];
            int jj = j + moves[k][1];
            
            if (valid(ii, jj, m, n)) {
                if (grid[ii][jj] == 0) {
                    walls += 1;
                    threat.add(ii * n + jj);
                } else if (grid[ii][jj] == 1 && !visited[ii][jj]) {
                    walls += dfs(ii, jj, grid, visited, threat, block);
                }
            }
        }
        
        return walls;
    }
    
    
    
    private boolean valid(int i, int j, int m, int n) {
        return i >= 0 && j >= 0 && i < m && j < n;
    }
}
