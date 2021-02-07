public class Solution {
    public int[][] candyCrush(int[][] board) {
        int m = board.length;
        int n = board[0].length;
        
        while (true) {
            boolean crush = false;
            
            for (int i = 0; i < m; i += 1) {
                for (int j = 0; j < n; j += 1) {
                    if (board[i][j] != 0) {
                        boolean v = (i > 0 && crush_v(board[i - 1][j]) && board[i][j] % 2000 == board[i - 1][j] % 2000) || (i + 2 < m && board[i][j] == board[i + 1][j] && board[i][j] == board[i + 2][j]);
                        boolean h = (j > 0 && crush_h(board[i][j - 1]) && board[i][j] % 2000 == board[i][j - 1] % 2000) || (j + 2 < n && board[i][j] == board[i][j + 1] && board[i][j] == board[i][j + 2]);
                        
                        if (v && h) {
                            board[i][j] += 6000;
                            crush = true;
                        } else if (v) {
                            board[i][j] += 2000;
                            crush = true;
                        } else if (h) {
                            crush = true;
                            board[i][j] += 4000;
                        }
                    }
                }
            }
            if (!crush) {
                break;
            }
            
            for (int j = 0; j < n; j += 1) {
                int p1 = m - 1;
                int p2 = m - 1;
                
                while (p1 >= 0) {
                    while (p2 >= 0 && (board[p2][j] == 0 || board[p2][j] > 2000)) {
                        p2 -= 1;
                    }
                    if (p2 < 0) {
                        board[p1][j] = 0;
                    } else {
                        board[p1][j] = board[p2][j];
                        p2 -= 1;
                    }
                    p1 -= 1;
                }
                
            }
        }
        
        return board;
        
        
    }
    
    private boolean crush_v(int x) {
        return (x > 2000 && x <= 4000) || x > 6000;
    }
    
    private boolean crush_h(int x) {
        return x > 4000;
    }
}
