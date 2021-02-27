use std::collections::HashMap;

impl Solution {
    pub fn count_corner_rectangles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        
        let mut count = 0;
        let mut map: HashMap<(usize, usize), i32> = HashMap::new();
        
        for i in 0..m {
            for j1 in 0..(n - 1) {
                if grid[i][j1] == 0 {
                    continue;
                }
                
                for j2 in (j1 + 1)..n {
                    if grid[i][j2] == 0 {
                        continue;
                    }
                    
                    match map.get_mut(&(j1, j2)) {
                        Some(prev) => {
                            count += *prev;
                            *prev = *prev + 1;
                        },
                        None => { map.insert((j1, j2), 1); },
                    }
                }
            }
        }
        count
    }
}
