pub fn count_homogenous(s: String) -> i32 {
    let (mo, n, cs, mut ans, mut l, mut r) = (
        1e9 as i64 + 7,
        s.len(),
        s.chars().collect::<Vec<char>>(),
        0,
        0,
        0,
    );
    while r < n {
        if cs[r] == cs[l] {
            r += 1;
        } else {
            let len = (r - l) as i64;
            ans += (len) * (len + 1) / 2;
            ans %= mo;
            l = r;
        }
    }
    let len = (r - l) as i64;
    ans += (len) * (len + 1) / 2;
    ans %= mo;
    ans as i32
}
