class Solution {
    public int[][] floodFill(int[][] image, int sr, int sc, int newColor) {
        int oldColor = image[sr][sc];
        if (oldColor == newColor) {
            return image;
        }
        dfs(image, sr, sc, oldColor, newColor);
        return image;
    }
    
    private void dfs(int[][] image, int r, int c, int oldColor, int newColor) {
        image[r][c] = newColor;
        int[][] moves = new int[][]{{0, 1}, {0, -1}, {1, 0}, {-1, 0}};
        for (int k = 0; k < 4; k += 1) {
            int nextr = r + moves[k][0];
            int nextc = c + moves[k][1];
            if (valid(nextr, nextc, image.length, image[0].length) && image[nextr][nextc] == oldColor) {
                dfs(image, nextr, nextc, oldColor, newColor);
            }
        }
    }
    
    private boolean valid(int i, int j, int m, int n) {
        return i >= 0 && j >= 0 && i < m && j < n;
    }
}
