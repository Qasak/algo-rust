// dp
pub fn min_distance(word1: String, word2: String) -> i32 {
    let bs1 = word1.as_bytes();
    let bs2 = word2.as_bytes();
    let (n, m) = (bs1.len(), bs2.len());
    let mut f = vec![vec![0; m + 1]; n + 1];
    for i in 0..=n {
        f[i][0] = i;
    }
    for j in 0..=m {
        f[0][j] = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            if bs1[i - 1] == bs2[j - 1] {
                f[i][j] = f[i - 1][j - 1];
            } else {
                // 修改其中一个 / (插入/删除)其中一个
                f[i][j] = f[i - 1][j - 1].min(f[i - 1][j].min(f[i][j - 1])) + 1;
            }
        }
    }

    f[n][m] as i32
}


// rec

