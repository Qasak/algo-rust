use std::collections::VecDeque;
pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let dirs = [(1, 0, 0), (0, 1, 0), (0, 0, 1)];
    let n: i32 = grid.len() as i32;
    let mut visit = vec![vec![vec![false; 2]; n as usize]; n as usize];
    visit[0][0][0] = true;
    let mut q = VecDeque::new();
    q.push_back((0, 0, 0));
    let mut step = 1;
    while !q.is_empty() {
        let len = q.len();
        for _ in 0..len {
            let t1 = q.pop_front().unwrap();
            for d in &dirs {
                let (x, y, s) = (t1.0 + d.0, t1.1 + d.1, t1.2 ^ d.2);
                // 🐍's head
                let (x2, y2) = (x + s, y + (s ^ 1));
                if x2 < n
                    && y2 < n
                    && !visit[x as usize][y as usize][s as usize]
                    && grid[x as usize][y as usize] == 0
                    && grid[x2 as usize][y2 as usize] == 0
                    && (d.2 == 0 || grid[(x + 1) as usize][(y + 1) as usize] == 0)
                {
                    if x == n - 1 && y == n - 2 {
                        return step;
                    }
                    visit[x as usize][y as usize][s as usize] = true;
                    q.push_back((x, y, s));
                }
            }
        }
        step += 1;
    }
    -1
}
