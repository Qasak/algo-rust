pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
    let mut max = 0;
    let mut ret = vec![0; 2];
    let n = towers.len();
    for x in 0..=50 {
        for y in 0..=50 {
            let mut cur = 0;
            for j in 0..n {
                let tr = &towers[j];
                let xx = tr[0]; let yy = tr[1];
                let signal = tr[2];
                let d = (((x - xx) * (x - xx) + (y - yy) * (y - yy)) as f32).sqrt();
                if d <= (radius as f32) {
                    // error cast `let w = signal / (1 + (d as i32));`
                    let w = ((signal as f32) / (1.0 + d)) as i32;
                    cur += w;
                }

            }
            if cur > max {
                max = cur;
                ret[0] = x; ret[1] = y;
            }
            if cur == max {
                if x < ret[0] {
                    ret[0] = x; ret[1] = y;
                } else if x == ret[0] && y < ret[1] {
                    ret[0] = x; ret[1] = y;
                }
            }
        }
    }
    ret
}
