pub fn reach_number(target: i32) -> i32 {
    let mut diff = target.abs();
    let mut step = 0;
    while diff > 0 {
        step += 1;
        diff -= step;
    }
    if diff % 2 == 0 {
        step
    } else {
        step + 1 + step % 2
    }
}
