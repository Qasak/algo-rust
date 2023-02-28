// back track
pub fn stone_game_ii(mut piles: Vec<i32>) -> i32 {
    let n = piles.len();
    // (i, m) (从下标i开始, 取m个)
    let mut cache = vec![vec![0; n]; n];
    let mut suf = piles;
    if n == 1 {
        return suf[0];
    }
    for i in (0..=(n - 2)).rev() {
        suf[i] += suf[i + 1];
    }
    dfs(0, 1, &suf, &mut cache)
}

fn dfs(i: usize, m: usize, suf: &Vec<i32>, cache: &mut Vec<Vec<i32>>) -> i32 {
    if i + 2 * m >= suf.len() {
        return suf[i];
    }
    if cache[i][m] != 0 {
        return cache[i][m];
    }
    let mut opponent = i32::MAX;
    // 自己最大 -> 对手最小
    for x in 1..=(2 * m) {
        opponent = opponent.min(dfs(i + x, m.max(x), suf, cache));
    }
    // 得分 = 当前后缀和 - 对手的最小得分
    cache[i][m] = suf[i] - opponent;
    cache[i][m]
}

// dp
pub fn stone_game_ii_dp(mut piles: Vec<i32>) -> i32 {
    let n = piles.len();
    // (i, m) (从下标i开始, 取m个)
    let mut suf = 0;
    let mut f = vec![vec![0; n + 1]; n];
    for i in (0..=(n - 1)).rev() {
        suf += piles[i];
        for m in 0..=piles.len() {
            if i + 2 * m >= piles.len() {
                f[i][m] = suf;
                continue;
            }
            let mut opponent = i32::MAX;
            for x in 1..=(2 * m) {
                opponent = opponent.min(f[i + x][m.max(x)]);
            }
            f[i][m] = suf - opponent;
        }
    }
    f[0][1]
}
