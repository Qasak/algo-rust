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

// bitwise
pub fn solve_n_queens_1(n: i32) -> Vec<Vec<String>> {
    fn dfs(ret: &mut Vec<Vec<String>>, path: &mut Vec<Vec<char>>, col: i32, d1: i32, d2: i32, i: usize) {
        let n = path.len();
        if i == n {
            ret.push(path.iter().map(|v| v.iter().map(|&c| c).collect::<String>()).collect::<Vec<String>>());
        } else {
            let mut pos = ((1 << n) - 1) & (!(col|d1|d2));
            while pos != 0 {
                let low_bit = pos & (-pos);
                pos = pos & (pos - 1);
                let j = (low_bit - 1).count_ones() as usize;
                path[i][j] = 'Q';
                dfs(ret, path, col|low_bit, (d1|low_bit) << 1, (d2|low_bit) >> 1, i + 1);
                path[i][j] = '.';
            }
        }
    }
    let n = n as usize;
    let mut ret = vec![];
    let mut path = vec![vec!['.'; n]; n];
    dfs(&mut ret, &mut path, 0, 0, 0, 0);
    ret
}


