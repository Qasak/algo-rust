fn num_islands(grid: Vec<Vec<char>>) -> i32 {

    fn dfs(i: usize, j: usize, vis: &mut Vec<Vec<bool>>, grid: &Vec<Vec<char>>) {
        vis[i][j] = true;
        for (ii, jj) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]{
            if ii >= 0 && ii < grid.len() && jj >= 0 && jj < grid[0].len() && !vis[ii][jj] && grid[ii][jj] == '1' {
                dfs(ii, jj, vis, grid);
            }
        }
    }
    let mut vis = vec![vec![false; grid[0].len()]; grid.len()];
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !vis[i][j] && grid[i][j] == '1' {
                dfs(i, j, &mut vis, &grid);
                res += 1;
            }
        }
    }
    res
}