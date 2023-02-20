pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    let (mut val, mut idx) = (i32::MAX, usize::MAX);
    for (i, p) in points.iter().enumerate() {
        if p[0] == x || p[1] == y {
            let cur = (p[0] - x).abs() + (p[1] - y).abs();
            if cur < val {
                idx = i;
                val = cur;
            }
        }
    }
    if idx == usize::MAX {
        -1
    } else {
        idx as i32
    }
}

// iter
pub fn nearest_valid_point_1(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    points
        .iter()
        .enumerate()
        .filter(|(i, p)| p[0] == x || p[1] == y)
        .fold((-1, i32::MAX), |ret, ip| {
            let cur = (ip.1[0] - x).abs() + (ip.1[1] - y).abs();
            if cur < ret.1 {
                (ip.0 as i32, cur)
            } else {
                ret
            }
        })
        .0
}
