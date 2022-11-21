use::std::collections::{VecDeque, HashSet};
pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let (mut q, mut vis, n, m) = (VecDeque::new(), HashSet::new(), maze.len() as i32, maze[0].len() as i32);
    vis.insert((entrance[0], entrance[1]));
    q.push_back((entrance[0], entrance[1], 0));
    while let Some((i, j, s)) = q.pop_front() {
        for (ii, jj) in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)] {
            if !vis.contains(&(ii, jj)) && ii >= 0 && ii < n && jj >= 0 && jj < m && maze[ii as usize][jj as usize] != '+' {
                if ii == 0 || ii == n - 1 || jj == 0 || jj == m - 1 {
                    return s + 1;
                }
                vis.insert((ii, jj));
                q.push_back((ii, jj, s + 1));
            }
        }
    }
    -1
}