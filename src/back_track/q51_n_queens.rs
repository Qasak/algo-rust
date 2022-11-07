pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    fn dfs(ret: &mut Vec<Vec<String>>, path: &mut Vec<Vec<char>>, vis: &mut Vec<Vec<bool>>, i: usize) {
        let n = path.len();
        if i == n {
            ret.push(path.iter().map(|v| v.iter().map(|&c| c).collect::<String>()).collect::<Vec<String>>());
        }
        for j in 0..n {
            if !vis[0][j] && !vis[1][i - j + n] && !vis[2][i + j] {
                path[i][j] = 'Q';
                vis[0][j] = true; vis[1][i - j + n] = true; vis[2][i + j] = true;
                dfs(ret, path, vis, i + 1);
                vis[0][j] = false; vis[1][i - j + n] = false; vis[2][i + j] = false;
                path[i][j] = '.';
            }
        }
    }
    let n = n as usize;
    let mut ret = vec![];
    let mut path = vec![vec!['.'; n]; n];
    let mut vis = vec![vec![false; 2 * n]; 3];
    dfs(&mut ret, &mut path, &mut vis, 0);
    ret
}

