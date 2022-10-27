pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
    let mut idxes = vec![];
    let n = img1.len();
    for i in 0..n {
        for j in 0..n {
            if img1[i][j] == 1 {
                idxes.push([i, j]);
            }
        }
    }
    let mut ret = 0;
    let n = n as i32;
    for x in (-(n - 1))..=(n - 1) {
        for y in (-(n - 1))..=(n - 1) {
            let mut cur = 0;
            for idx in &idxes {
                let i = idx[0] as i32 + x;
                let j = idx[1] as i32 + y;
                if i >= 0 && i < n && j >= 0 && j < n {
                    if img2[i as usize][j as usize] == 1 {
                        cur += 1;
                    }
                }

            }
            ret = ret.max(cur);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use crate::simulation::q835_image_overlap::largest_overlap;
    #[test]
    pub fn t() {
        // let img1  = [vec![1,1,0],vec![0,1,0],vec![0,1,0]].to_vec();
        // let img2  = [vec![0,0,0],vec![0,1,1],vec![0,0,1]].to_vec();
        let img1  = [vec![1]].to_vec();
        let img2  = [vec![1]].to_vec();
        println!("{:?}", largest_overlap(img1, img2));
    }
}