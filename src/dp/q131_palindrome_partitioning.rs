// dp + dfs bf
pub fn partition(s: String) -> Vec<Vec<String>> {
    // f[i][j] = cs[i] == cs[j] && f[i + 1][j - 1]
    let cs = s.chars().collect::<Vec<char>>();
    let n = s.len();
    let mut f = vec![vec![false; n]; n];
    let mut ret = vec![];
    for i in 0..n {
        f[i][i] = true;
    }
    for len in 2..(n + 1) {
        for i in 0..n {
            let j = i + len - 1;
            if j >= n {
                break;
            }
            if j - i == 1 {
                f[i][j] = cs[i] == cs[j];
            } else if j - i > 1 {
                f[i][j] = cs[i] == cs[j] && f[i + 1][j - 1];
            }
        }
    }
    fn dfs(
        start: usize,
        cs: &Vec<char>,
        ret: &mut Vec<Vec<String>>,
        path: &mut Vec<String>,
        f: &Vec<Vec<bool>>,
    ) {
        let n = cs.len();
        if start == n {
            ret.push(path.clone());
            return;
        }
        for i in start..n {
            if f[start][i] {
                path.push(cs[start..(i + 1)].iter().collect::<String>());
                dfs(i + 1, cs, ret, path, f);
                path.pop();
            }
        }
    }
    dfs(0, &cs, &mut ret, &mut vec![], &f);
    ret
}
