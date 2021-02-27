class Solution {
    public int countCornerRectangles(int[][] grid) {
        int m = grid.length;
        int n = grid[0].length;
        
        int count = 0;
        
        Map<Integer, Integer>  map = new HashMap<>();
        for (int i = 0; i < m; i += 1) {
            for (int j1 = 0; j1 < n - 1; j1 += 1) {
                if (grid[i][j1] == 0) {
                    continue;
                }
                
                for (int j2 = j1 + 1; j2 < n; j2 += 1) {
                    if (grid[i][j2] == 0) {
                        continue;
                    }
                    int key = j1 * 200 + j2;
                    if (map.containsKey(key)) {
                        int prev = map.get(key);
                        count += prev;
                        map.replace(key, prev + 1);
                    } else {
                        map.put(key, 1);
                    }
                }
            }
        }
        return count;
    }
}
