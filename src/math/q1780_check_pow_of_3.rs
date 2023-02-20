// fxxking bf
pub fn check_powers_of_three(mut n: i32) -> bool {
    let mut v = vec![];
    for i in 0..15 {
        v.push(i32::pow(3, i));
    }
    fn dfs(i: usize, v: &Vec<i32>, n: i32, cur: i32) -> bool {
        if cur == n {
            return true;
        }
        if i == v.len() || cur > n {
            return false;
        }
        return dfs(i + 1, v, n, cur + v[i]) | dfs(i + 1, v, n, cur);
    }
    dfs(0, &v, n, 0)
}

// consider 'X' system like binary/hex/decimal...
pub fn check_powers_of_three_1(mut n: i32) -> bool {
    while n > 0 {
        if n % 3 == 2 {
            return false;
        }
        n /= 3;
    }
    true
}
