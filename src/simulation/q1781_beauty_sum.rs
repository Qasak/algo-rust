pub fn beauty_sum(s: String) -> i32 {
    let cs = s.bytes().collect::<Vec<u8>>();
    let n = cs.len();
    let mut cnt = vec![vec![0; 26]; n + 1];
    for i in 0..n {
        for k in 0..26 {
            cnt[i + 1][k] = cnt[i][k] + if cs[i] == k as u8 + b'a' {1} else {0};
        }
    }
    let mut ret = 0;
    for i in 0..n {
        for j in 0..i {
            let (mut min, mut max) = (u32::MAX, 0);
            for k in 0..26 {
                let c = cnt[i + 1][k] - cnt[j][k];
                if c != 0 {
                    min = min.min(c);
                    max = max.max(c);
                }
            }
            ret += max - min;
        }
    }
    ret as i32
}
