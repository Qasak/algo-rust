pub fn magical_string(n: i32) -> i32 {
    let n = n as usize;
    let mut v = vec![0; n + 2];
    v[0] = 1; v[1] = 2; v[2] = 2;
    let mut j = 3;
    for i in 2..n {
        if j >= n {
            break;
        }
        let num = if i % 2 == 0 {1} else {2};
        if v[i] == 2 {
            v[j] = num;
            v[j + 1] = num;
            j += 2;
        } else {
            v[j] = num;
            j += 1;
        }
    }
    let mut cnt = 0;
    for i in 0..n {
        if v[i] == 1 {
            cnt += 1;
        }
    }
    cnt
}