// dp + dfs bf
pub fn min_cut(s: String) -> i32 {
    // f[i][j] = cs[i] == cs[j] && f[i + 1][j - 1]
    let cs = s.chars().collect::<Vec<char>>();
    let n = s.len();
    let mut f = vec![vec![false; n]; n];
    for i in 0..n {
        f[i][i] = true;
    }
    for len in 2..(n + 1) {
        for i in 0..n {
            let j = i + len - 1;
            if j >= n {break;}
            if j - i == 1 {
                f[i][j] = cs[i] == cs[j];
            } else if j - i > 1{
                f[i][j] = cs[i] == cs[j] && f[i + 1][j - 1];
            }
        }
    }
    fn dfs(start: usize, f: &Vec<Vec<bool>>, cnt: i32, ans: &mut i32) {
        let n = f.len();
        if start == n {
            *ans = (*ans).min(cnt - 1);
            return;
        }
        for i in start..n {
            if f[start][i] {
                dfs(i + 1, f, cnt + 1, ans);
            }
        }
    }
    let mut ans = i32::MAX;
    dfs(0, &f, 0, &mut ans);
    ans
}

// dp + dp LCS
pub fn min_cut_1(s: String) -> i32 {
    // f[i][j] = cs[i] == cs[j] && f[i + 1][j - 1]
    let cs = s.chars().collect::<Vec<char>>();
    let n = s.len();
    let mut f = vec![vec![false; n]; n];
    for i in 0..n {
        f[i][i] = true;
    }
    for len in 2..(n + 1) {
        for i in 0..n {
            let j = i + len - 1;
            if j >= n {break;}
            if j - i == 1 {
                f[i][j] = cs[i] == cs[j];
            } else if j - i > 1{
                f[i][j] = cs[i] == cs[j] && f[i + 1][j - 1];
            }
        }
    }
    let mut g = vec![i32::MAX - 1; n];
    g[0] = 0;
    // end at index i
    for i in 0..n {
        for j in 0..(i + 1) {
            if f[j][i] {
                if j > 0 {
                    g[i] = g[i].min(g[j - 1] + 1);
                } else {
                    g[i] = 0;
                }
            }
        }
    }
    // println!("{:?}", g);
    g[n - 1]
}