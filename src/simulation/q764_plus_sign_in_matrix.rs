// bf
pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut grid = vec![vec![1; n]; n];
    for m in mines {
        grid[m[0] as usize][m[1] as usize] = 0;
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] != 1 {
                continue;
            }
            let mut t = vec![0; 4];
            for k in (0..j).rev() {
                if grid[i][k] == 0 {
                    break;
                }
                t[0] += 1;
            }
            for k in (j + 1)..n {
                if grid[i][k] == 0 {
                    break;
                }
                t[1] += 1;
            }
            for k in (0..i).rev() {
                if grid[k][j] == 0 {
                    break;
                }
                t[2] += 1;
            }
            for k in (i + 1)..n {
                if grid[k][j] == 0 {
                    break;
                }
                t[3] += 1;
            }
            ans = ans.max(1 + t.into_iter().min().unwrap());
        }
    }
    ans
}
