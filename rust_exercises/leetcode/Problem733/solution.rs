impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let old_color = image[sr as usize][sc as usize];
        if old_color == new_color {
            return image;
        }
        dfs(&mut image, sr, sc, old_color, new_color);
        image
    }
}

fn dfs(image: &mut Vec<Vec<i32>>, r: i32, c: i32, old_color: i32, new_color: i32) {
    image[r as usize][c as usize] = new_color;
    
    let moves = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for (mr, mc) in moves.iter() {
        let nextr = r + mr;
        let nextc = c + mc;
        
        if valid(nextr, nextc, image.len(), image[0].len()) && image[nextr as usize][nextc as usize] == old_color {
            dfs(image, nextr, nextc, old_color, new_color);
        }
    }
}

fn valid(i: i32, j: i32, m: usize, n: usize) -> bool {
    i >= 0 && j >= 0 && i < m as i32 && j < n as i32
}
