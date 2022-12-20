pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
    fn is_force_big(pre: &Vec<i64>, target_balls: i32, force: i32) -> bool {
        let (mut l, mut r, mut balls) = (0, 1, 0);
        while r < pre.len() {
            if pre[r] - pre[l] >= force as i64 {
                l = r;
                balls += 1;
            }
            r += 1;
        }
        balls + 1 < target_balls
    }
    position.sort();
    let mut pre = vec![0; position.len()];
    for i in 2..=pre.len() {
        pre[i - 1] = pre[i - 2] + (position[i - 1] - position[i - 2]) as i64;
    }
    let (mut l, mut r) = (1, position[position.len() - 1]);
    while l < r {
        let f = l + (r - l + 1) / 2;
        if is_force_big(&pre, m, f) {
            r = f - 1;
        } else {
            l = f;
        }
    }
    l
}