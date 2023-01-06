pub fn most_points_1(questions: Vec<Vec<i32>>) -> i64 {
    let n = questions.len();
    let mut f = vec![0i64; n];
    for i in (0..n) {
        let (p, b) = (questions[i][0] as i64, questions[i][1] as usize);
        let j = i + 1;
        if j < n {
            f[j] = f[j].max(f[i]);
        }
        f[i] += p;
        let j = i + b + 1;
        if j < n {
            f[j] = f[j].max(f[i]);
        }
        // println!("f: {:?}", f);
    }
    f.into_iter().max().unwrap_or(0)
}
pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let n = questions.len();
    let mut f = vec![0i64; n];
    f[n - 1] = questions[n - 1][0] as i64;
    for i in (0..n - 1).rev() {
        let (p, b) = (questions[i][0] as i64,  questions[i][1] as usize);
        let j = i + b + 1;
        f[i] = if j >= n {
            p
        } else {
            p + f[j]
        }.max(f[i + 1]);
    }
    f.into_iter().max().unwrap_or(0)
}