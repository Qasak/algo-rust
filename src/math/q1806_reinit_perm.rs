// naive simulate
pub fn reinitialize_permutation(n: i32) -> i32 {
    let n = n as usize;
    let perm: Vec<usize> = (0..n).into_iter().collect();
    let mut arr = perm.clone();
    let mut ret = 0;
    loop {
        let mut t = arr.clone();
        for i in 0..n {
            if i % 2 == 0 {
                arr[i] = t[i / 2];
            } else {
                arr[i] = t[n / 2 + (i - 1) / 2]
            }
        }
        ret += 1;
        if arr == perm {
            break;
        }
    }
    ret
}

// math: find the inverse function of indexes
// f1(i) = i / 2
// f2(i) = n / 2 + (i - 1) / 2

// g1(i) = 2 * i
// g2(i) = 2 * i - n + 1
pub fn reinitialize_permutation_1(n: i32) -> i32 {
    let mut i = 1;
    let mut ret = 0;
    loop {
        ret += 1;
        if i <= (n - 1) / 2 {
            i = i * 2;
        } else {
            i = i * 2 - (n - 1);
        }
        if i == 1 {
            break;
        }
    }
    ret
}
