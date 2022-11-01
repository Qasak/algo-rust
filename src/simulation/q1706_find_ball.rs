pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let m = grid[0].len();
    let mut ret = vec![-1; m];
    for i in 0..m {
        let (mut x, mut y) = (0, i);
        while x < n {
            let d = grid[x][y];
            let yy = y as i32 + d;
            if yy as usize >= m || yy < 0 || grid[x][yy as usize] == -d {
                break;
            }
            x += 1;
            y = yy as usize;
        }
        if x == n {
            ret[i] = y as i32;
        }
    }
    ret
}
