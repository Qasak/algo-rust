fn balanced_string(s: String) -> i32 {
    let s = s.as_bytes();
    let mut cnt = [0; 128];
    for &c in s {
        cnt[c as usize] += 1;
    }
    let n = s.len();
    let m = n / 4;
    if cnt['Q' as usize] == m
        && cnt['W' as usize] == m
        && cnt['E' as usize] == m
        && cnt['R' as usize] == m
    {
        return 0;
    }
    let mut ans = n;
    let mut left = 0;
    for right in 0..n {
        cnt[s[right] as usize] -= 1;
        while cnt['Q' as usize] <= m
            && cnt['W' as usize] <= m
            && cnt['E' as usize] <= m
            && cnt['R' as usize] <= m
        {
            ans = ans.min(right - left + 1);
            cnt[s[left] as usize] += 1;
            left += 1;
        }
    }
    ans as i32
}
