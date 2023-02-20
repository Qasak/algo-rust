// dp solution
// math analysis: https://leetcode.com/problems/soup-servings/solutions/195582/a-mathematical-analysis-of-the-soup-servings-problem/
pub fn soup_servings(n: i32) -> f64 {
    if n >= 5500 {
        return 1.0;
    }
    let n = ((n as f64) / 25.0).ceil() as usize;
    // f: the probability that soup A will be empty `first`,
    // plus half the probability that A and B become empty at the same time.
    let mut f = vec![vec![0.0; n + 1]; n + 1];
    f[0][0] = 0.5;
    for i in 1..=n {
        f[0][i] = 1.0;
    }
    for i in 1..=n {
        for j in 1..=n {
            let i = i as i32;
            let j = j as i32;
            f[i as usize][j as usize] = 0.25
                * (f[(i - 4).max(0) as usize][j as usize]
                    + f[(i - 3).max(0) as usize][(j - 1).max(0) as usize]
                    + f[(i - 2).max(0) as usize][(j - 2).max(0) as usize]
                    + f[(i - 1).max(0) as usize][(j - 3).max(0) as usize])
        }
    }
    f[n][n]
}
