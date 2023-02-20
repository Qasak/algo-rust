pub fn super_egg_drop(k: i32, n: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut g = vec![vec![100; n + 1]; k + 1];
    // only 1 🥚
    for i in 1..=n {
        g[0][i] = 0;
        g[1][i] = i;
    }
    // only 1 level
    g[0][1] = 0;
    g[0][0] = 0;
    for i in 1..=k {
        g[i][0] = 0;
        g[i][1] = 1;
    }
    // println!("{:?}", g);
    // i🥚
    for i in 2..=k {
        // j level
        for j in 2..=n {
            // search x which makes max(boom, good) min
            // (]
            let mut l = 1;
            let mut r = j;
            while l < r {
                let x = l + (r - l + 1) / 2;
                let boom = g[i - 1][x - 1];
                let good = g[i][j - x];
                if boom > good {
                    r = x - 1;
                } else {
                    l = x;
                }
            }
            g[i][j] = g[i - 1][l - 1].max(g[i][j - l]) + 1;
        }
    }
    // println!("{:?}", g);
    g[k][n] as i32
}
