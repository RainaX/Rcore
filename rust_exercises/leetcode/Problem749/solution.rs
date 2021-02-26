use std::collections::HashSet;

impl Solution {
    pub fn contain_virus(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        
        let mut count_walls = 0;
        loop {
            let mut visited = vec![vec![false; n]; m];
            let mut max_threat = HashSet::new();
            let mut max_block = HashSet::new();
            let mut max_walls = 0;
                    
            let mut to_infect = HashSet::new();
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 && !visited[i][j] {
                        let mut threat = HashSet::new();
                        let mut block = HashSet::new();
                        let walls = dfs(i, j, &mut grid, &mut visited, &mut threat, &mut block);
                        
                        if threat.len() > max_threat.len() {
                            for x in max_threat.into_iter() {
                                to_infect.insert(x);
                            }
                            max_threat = threat;
                            max_block = block;
                            max_walls = walls;
                        } else {
                            for x in threat.into_iter() {
                                to_infect.insert(x);
                            }
                        }
                    }
                }
            }
            
            if max_threat.is_empty() {
                break;
            }
            for (i, j) in to_infect.into_iter() {
                grid[i][j] = 1;
            }
            for (i, j) in max_block.into_iter() {
                grid[i][j] = 2;
            }
            count_walls += max_walls;
        }
        
        count_walls
    }
}

fn valid(i: i32, j: i32, m: usize, n: usize) -> bool {
    i >= 0 && j >= 0 && (i as usize) < m && (j as usize) < n
}

fn dfs(i: usize, j: usize, grid: &mut Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, threat: &mut HashSet<(usize, usize)>, block: &mut HashSet<(usize, usize)>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    
    visited[i][j] = true;
    block.insert((i, j));
    
    let moves = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut walls = 0;
    
    for (di, dj) in moves.into_iter() {
        let ii = i as i32 + di;
        let jj = j as i32 + dj;
        
        if valid(ii, jj, m, n) {
            let ii = ii as usize;
            let jj = jj as usize;
            if grid[ii][jj] == 0 {
                walls += 1;
                threat.insert((ii, jj));
            } else if grid[ii][jj] == 1 && !visited[ii][jj] {
                walls += dfs(ii, jj, grid, visited, threat, block);
            }
        }
    }
    
    walls
}
