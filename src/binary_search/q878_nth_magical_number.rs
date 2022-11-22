pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
    let g = 1e9 as i64 + 7;
    let (a, b, n) = (a as i64, b as i64, n as i64);
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {a} else {gcd(b, a % b)}
    }
    let lcm = a * b / gcd(a, b);
    let (mut l, mut r) = (1 as i64, a.min(b) * n + 1);
    while l < r {
        let m = l + (r - l) / 2;
        let nth = m / a + m / b - m / lcm;
        if nth < n {
            l = m + 1;
        } else {
            r = m;
        }
    }
    (l % g) as i32
}