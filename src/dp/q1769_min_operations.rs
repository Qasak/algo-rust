// bf
pub fn min_operations(boxes: String) -> Vec<i32> {
    let n = boxes.len();
    let mut ret = vec![0; n];
    let cs = boxes.as_bytes();
    for i in 0..n {
        for j in 0..n {
            if cs[j] == b'1' {
                ret[i] += (i as i32 - j as i32).abs();
            }
        }
    }
    ret
}

// dp
pub fn min_operations_1(boxes: String) -> Vec<i32> {
    let n = boxes.len();
    // f: ith's operation count
    let (mut f, cs) = (vec![0; n], boxes.as_bytes());
    // l: num of ith left '1's
    // r: num of ith right '1's
    // f[i] = f[i - 1] + l - r
    let (mut l, mut r) = (0 as i32, 0 as i32);
    for i in 1..n {
        if cs[i] == b'1' {
            r += 1;
            f[0] += i as i32;
        }
    }
    if cs[0] == b'1' {l = 1;}
    for i in 1..n {
        f[i] = f[i - 1] + l - r;
        if cs[i] == b'1' {
            l += 1; r -= 1;
        }
    }
    f
}
