use std::collections::{HashSet, VecDeque};


// todo!
// Bugs to be fixed soon
pub fn shortest_path_dfs(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let k = k as usize;
    let mut vis = vec![vec![false; m]; n];
    // (i, j, k) 出发状态到达[n - 1, m - 1]需要的最小步骤
    let mut f = vec![vec![vec![0x3f3f3f3f; k + 1]; m]; n];
    // (i, j, k) 出发状态到达[n - 1, m - 1]需要的最小步骤
    fn dfs(i: usize, j: usize, k: usize, grid: &mut Vec<Vec<i32>>, vis: &mut Vec<Vec<bool>>, f: &mut Vec<Vec<Vec<i32>>> ) -> i32 {

        let n = grid.len();
        let m = grid[0].len();
        let dirs = [[-1, 0], [0, 1], [1, 0], [0, -1]];
        let mut min = 0x3f3f3f3f;
        // > || < return
        if f[i][j][k] != 0x3f3f3f3f {
            // println!("i: {}, j: {}, k: {}, f[i][j][k]: {}", i, j, k, f[i][j][k]);
            return f[i][j][k];
        }
        if i == grid.len() - 1 && j == grid[0].len() - 1 {
            return 0;
        }
        vis[i][j] = true;
        for d in dirs {
            let ii = i as i32 + d[0];
            let jj = j as i32 + d[1];
            if ii >= 0 && ii < n as i32 && jj >= 0 && jj < m as i32 && !vis[ii as usize][jj as usize] {

                if grid[ii as usize][jj as usize] == 1 {
                    if k > 0 {
                        // grid[ii as usize][jj as usize] = 0;
                        min = min.min(dfs(ii as usize, jj as usize, k - 1, grid, vis, f));
                        // grid[ii as usize][jj as usize] = 1;
                    }
                } else {
                    min = min.min(dfs(ii as usize, jj as usize, k, grid, vis, f));
                }
            }
        }
        vis[i][j] = false;
        // 反向计数
        // backtrack counting
        f[i][j][k] = min + 1;
        f[i][j][k]
    }
    let res = dfs(0, 0, k, &mut grid, &mut vis, &mut f);
    // println!("{}", f[0][0][k]);
    if res >= 0x3f3f3f3f {-1} else {res}
}

pub fn shortest_path_bfs(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let (n, m) = (grid.len() as i32, grid[0].len() as i32);
    // good prune!
    if k >= n + m - 2 { return n + m - 2; }
    let mut dq = VecDeque::from([(0,0,k,0)]);
    let mut seen = HashSet::new();
    while let Some((i,j,k,step)) = dq.pop_front(){
        if i == n-1 && j == m-1 { return step; }
        // nice condition!
        for (ii,jj) in vec![(i+1,j),(i-1,j),(i,j+1),(i,j-1)]{
            if (0 <= ii && ii < n && 0 <= jj && jj < m){
                // nice try!
                let kk = k - grid[ii as usize][jj as usize];
                if kk >= 0{
                    if !seen.contains(&(ii, jj, kk)){
                        seen.insert((ii, jj, kk));
                        dq.push_back((ii, jj, kk, step+1));
                    }
                }
            }
        }
    }
    return -1;
}