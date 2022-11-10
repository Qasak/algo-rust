// dfs bf (TLE)
pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
    let (n, m) = (grid.len(), grid[0].len());
    let mut g = vec![];
    let mut keys = Vec::new();
    for row in grid {
        let mut path = vec![];
        for ch in row.chars() {
            path.push(ch);
            if ch.is_ascii_lowercase() {
                keys.push(ch);
            }
        }
        g.push(path);
    }
    keys.sort();
    // (i, j, cur_keys, step)
    let mut vis = HashSet::new();
    let mut ans = i32::MAX;
    let mut cur_keys = vec![];
    for i in 0..n {
        for j in 0..m {
            if g[i][j] == '@' {
                vis.insert((i, j, Vec::new()));
                dfs(i, j, &mut g, &mut vis, &mut cur_keys, &mut keys, 0, &mut ans);
                break;
            }
        }
    }
    fn dfs(i: usize, j : usize, g: &mut Vec<Vec<char>>, vis: &mut HashSet<(usize, usize, Vec<char>)>, cur_keys: &mut Vec<char>, keys: &Vec<char>, step: i32, ans: &mut i32) {
        // if cur_keys.len() >= 6 {
        //     println!("{:?}", ((i, j), &cur_keys, step));
        // }
        let (n, m) = (g.len(), g[0].len());
        let (mut k1, mut k2) = (cur_keys.clone(), keys.clone());
        k1.sort(); k2.sort();
        if k1 == k2 {
            // println!("{:?}", ((i, j), &cur_keys, step));
            *ans = (*ans).min(step);
            return;
        }

        for (ii, jj) in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)] {
            if ii >= 0 && ii < n && jj >= 0 && jj < m {
                let nxt_grid = g[ii][jj];
                // can't move situations is:
                // nxt_grid
                // 1. #
                // 2. no match key to unlock

                if nxt_grid == '#' || (nxt_grid.is_ascii_uppercase() && !cur_keys.contains(&(nxt_grid.to_ascii_lowercase()))) {
                    continue;
                }
                let mut pick = false;
                if nxt_grid.is_ascii_lowercase()  {
                    pick = true;
                }
                if !vis.contains(&(ii, jj, cur_keys.clone())) {
                    let t = g[ii][jj];
                    if pick {
                        cur_keys.push(nxt_grid);
                        g[ii][jj] = '.';
                    }
                    vis.insert((ii, jj, cur_keys.clone()));
                    dfs(ii, jj, g, vis, cur_keys, keys, step + 1, ans);
                    vis.remove(&(ii, jj, cur_keys.clone()));
                    if pick {
                        cur_keys.pop();
                        g[ii][jj] = t;
                    }
                }
            }
        }
    }
    if ans == i32::MAX {-1} else {ans}
}