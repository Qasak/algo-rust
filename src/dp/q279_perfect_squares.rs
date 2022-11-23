pub fn num_squares(n: i32) -> i32 {
    let n = n as usize;
    let m = (n as f64).sqrt() as usize + 1;
    let mut f = vec![i32::MAX - 1; n + 1];
    f[0] = 0;
    for i in 0..=n {
        for j in 1..m {
            if j * j <= i {
                f[i] = f[i].min(f[i - j * j] + 1);
            }
        }
    }
    f[n]
}

pub fn num_squares_1(n: i32) -> i32 {
    let n = n as usize;
    let mut f = vec![i32::MAX - 1; n + 1];
    f[0] = 0;
    for i in 1..=n {
        f[i] = (1..((i as f64).sqrt() as usize + 1))
            .map(|j| f[i - (j * j)] + 1)
            .min()
            .unwrap();
    }
    f[n]
}