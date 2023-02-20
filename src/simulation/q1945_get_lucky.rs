pub fn get_lucky(s: String, k: i32) -> i32 {
    fn f(cur: &mut Vec<i32>, mut cnt: i32) {
        while cnt > 0 {
            cur.push(cnt % 10);
            cnt /= 10;
        }
    }

    let mut cur = vec![];
    for b in s.bytes() {
        let mut cnt = (b - b'a') as i32 + 1;
        f(&mut cur, cnt);
    }
    for i in 0..(k - 1) {
        let mut cnt = cur.iter().sum::<i32>();
        cur.clear();
        f(&mut cur, cnt);
    }
    cur.iter().sum::<i32>()
}

// iterate
pub fn get_lucky_1(s: String, k: i32) -> i32 {
    (0..(k - 1)).fold(
        s.bytes()
            .map(|b| (b - b'a') as i32 + 1)
            .map(|i| i / 10 + i % 10)
            .sum(),
        |cnt, _| cnt.to_string().bytes().map(|b| (b - b'0') as i32).sum(),
    )
}
