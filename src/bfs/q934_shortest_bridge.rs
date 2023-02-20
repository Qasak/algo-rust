use ::std::collections::VecDeque;

pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len() as i32;
    let dir = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, dir: &[[i32; 2]; 4]) {
        if grid[i][j] == 0 || grid[i][j] == 2 {
            return;
        }
        grid[i][j] = 2;
        let n: i32 = grid.len() as i32;
        for d in dir {
            let ii = i as i32 + d[0];
            let jj = j as i32 + d[1];
            if ii >= 0 && ii < n && jj >= 0 && jj < n {
                dfs(grid, ii as usize, jj as usize, dir);
            }
        }
    }
    let mut flag = false;
    for i in 0..n {
        if flag {
            break;
        }
        for j in 0..n {
            if grid[i as usize][j as usize] == 1 {
                dfs(&mut grid, i as usize, j as usize, &dir);
                flag = true;
                break;
            }
        }
    }
    let mut q = VecDeque::new();
    for i in 0..n {
        for j in 0..n {
            if grid[i as usize][j as usize] == 2 {
                q.push_back(vec![i, j]);
            }
        }
    }
    let mut ret = 0;
    while !q.is_empty() {
        let m = q.len();
        for i in 0..m {
            let cur = q.pop_front().unwrap();
            let x = cur[0];
            let y = cur[1];
            for d in dir {
                let xx = x + d[0];
                let yy = y + d[1];
                if xx >= 0 && xx < n && yy >= 0 && yy < n {
                    if grid[xx as usize][yy as usize] == 1 {
                        return ret;
                    }
                    if grid[xx as usize][yy as usize] == 0 {
                        grid[xx as usize][yy as usize] = 2;
                        q.push_back(vec![xx, yy]);
                    }
                }
            }
        }
        ret += 1;
    }
    ret
}
