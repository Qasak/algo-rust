use std::collections::{HashMap, HashSet, VecDeque};
pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut qs = vec![];
    let n = queries.len();
    let mut cache = HashMap::new();
    for (i, q) in (queries).into_iter().enumerate() {
        qs.push((q, i));
        cache.insert(q, -1);
    }
    qs.sort();
    let mut f = vec![0; n + 1];
    let mut ready = HashSet::new();
    ready.insert((0, 0));
    let (nn, mm) = (grid.len(), grid[0].len());
    let mut vis = vec![vec![false; mm]; nn];
    // println!("cache {:?}", cache);

    for i in 1..=n {
        let cnt;
        if cache[&qs[i - 1].0] != -1 {
            // println!("cached {}", i);
            cnt = cache[&qs[i - 1].0];
        } else {
            cnt = bfs(&mut ready, &grid, &mut vis, qs[i - 1].0);
            *cache.entry(qs[i - 1].0).or_default() = f[i - 1] + cnt;
        };
        if i >= 2 && qs[i - 1].0 > qs[i - 2].0 {
            f[i] = f[i - 1] + cnt;
        } else {
            f[i] = cnt;
        }
        // f[i] = cnt;
    }
    let mut ret = vec![0 as i32; n];
    for (i, q) in qs.into_iter().enumerate() {
        ret[q.1] = f[i + 1];
    }
    ret
}
fn bfs(
    mut ready: &mut HashSet<(usize, usize)>,
    grid: &Vec<Vec<i32>>,
    vis: &mut Vec<Vec<bool>>,
    cur: i32,
) -> i32 {
    // println!("{}", cur);
    let mut q = VecDeque::new();
    for &pair in ready.iter() {
        q.push_back(pair);
    }
    ready.clear();
    let (n, m) = (grid.len(), grid[0].len());
    let mut step = 0;
    while !q.is_empty() {
        let len = q.len();
        for k in 0..len {
            let (i, j) = q.pop_front().unwrap();
            if grid[i][j] >= cur {
                ready.insert((i, j));
                continue;
            }
            if vis[i][j] {
                continue;
            }
            vis[i][j] = true;
            // println!("vised {:?}", (i, j));
            step += 1;
            for (ii, jj) in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)] {
                if ii >= 0 && ii < n && jj >= 0 && jj < m && !vis[ii][jj] {
                    q.push_back((ii, jj));
                }
            }
        }
    }
    // println!("ready: {:?}", ready);
    // println!("step: {:?}", step);
    step
}
