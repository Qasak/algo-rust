pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut right_most = vec![0; n + 1];
    for i in 0..=n {
        right_most[i] = i;
    }
    for i in 0..=n {
        let start = 0.max(i as i32 - ranges[i]);
        let end = (n as i32).min(i as i32 + ranges[i]);
        let start = start as usize;
        let end = end as usize;
        right_most[start] = right_most[start].max(end);
    }
    let mut last = 0;
    let mut ret = 0;
    let mut pre = 0;
    for i in 0..n {
        last = last.max(right_most[i]);
        if i == last {
            return -1;
        }
        if i == pre {
            ret += 1;
            pre = last;
        }
    }
    ret
}
