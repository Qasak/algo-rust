pub fn minimum_length(s: String) -> i32 {
    let (n, mut cs) = (s.len() as i32, s.chars().collect::<Vec<char>>());
    let (mut l, mut r) = (0 as i32, n as i32 - 1);
    while l < r && cs[l as usize] == cs[r as usize] {
        let c = cs[l as usize];
        while l < n && cs[l as usize] == c {
            l += 1;
        }
        while r >= 0 && cs[r as usize] == c {
            r -= 1;
        }
    }
    if (r - l) < 0 {0} else {r - l + 1}
}