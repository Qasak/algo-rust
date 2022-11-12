// find the pattern
pub fn num_tilings(n: i32) -> i32 {
    let n = n as usize;
    let mut f = vec![0; n + 4];
    f[1] = 1; f[2] = 2; f[3] = 5;
    let m = 1_000_000_007;
    for i in 4..=n {
        f[i] = ((2 * f[i - 1]) % m + f[i - 3] % m ) % m;
    }
    f[n] % m
}