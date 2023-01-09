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