impl Solution {
    pub fn candy_crush(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut board = board;
        let m = board.len();
        let n = board[0].len();
        
        loop {
            let mut crush = false;
            
            for i in 0..m {
                for j in 0..n {
                    if board[i][j] != 0 {
                        let v = (i > 0 && crush_v(board[i - 1][j]) && board[i][j] % 2000 == board[i - 1][j] % 2000) || (i + 2 < m && board[i][j] == board[i + 1][j] && board[i][j] == board[i + 2][j]);
                        let h = (j > 0 && crush_h(board[i][j - 1]) && board[i][j] % 2000 == board[i][j - 1] % 2000) || (j + 2 < n && board[i][j] == board[i][j + 1] && board[i][j] == board[i][j + 2]);
                        
                        if v && h {
                            board[i][j] += 6000;
                            crush = true;
                        } else if v {
                            board[i][j] += 2000;
                            crush = true;
                        } else if h {
                            board[i][j] += 4000;
                            crush = true;
                        }
                    }
                }
            }
            
            
            
            if !crush {
                break;
            }
            
            for j in 0..n {
                let mut p1: isize = m as isize - 1;
                let mut p2: isize = m as isize - 1;
                
                while p1 >= 0 {
                    while p2 >= 0 && (board[p2 as usize][j] == 0 || board[p2 as usize][j] > 2000) {
                        p2 -= 1;
                    }
                    
                    
                    if (p2 < 0) {
                        board[p1 as usize][j] = 0;
                    } else {
                        board[p1 as usize][j] = board[p2 as usize][j];
                        p2 -= 1;
                    }
                    
                    p1 -= 1;
                }
            }
            
        }
        
        board
    }
}

fn crush_v(x: i32) -> bool {
    (x > 2000 && x <= 4000) || x > 6000
}

fn crush_h(x: i32) -> bool {
    x > 4000
}
