pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let n = questions.len();
    let mut f = vec![0i64; n];
    for i in (0..n) {
        let (p, b) = (questions[i][0] as i64, questions[i][1]);
        let j = i + 1;
        if j < n {
            f[j] = f[j].max(f[i]);
        }
        f[i] += p;
        let j = i + b as usize + 1;
        if j < n {
            f[j] = f[j].max(f[i]);
        }
        // println!("f: {:?}", f);
    }
    *f.iter().max().unwrap()
}