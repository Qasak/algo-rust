pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
    fn is_force_big(pos: &Vec<i32>, target_balls: i32, force: i32) -> bool {
        let (mut l, mut balls) = (0, 0);
        for r in 0..pos.len() {
            if pos[r] - pos[l] >= force {
                l = r;
                balls += 1;
            }
        }
        balls + 1 < target_balls
    }
    position.sort();
    let (mut l, mut r) = (1, position[position.len() - 1]);
    while l < r {
        let f = l + (r - l + 1) / 2;
        if is_force_big(&position, m, f) {
            r = f - 1;
        } else {
            l = f;
        }
    }
    l
}
