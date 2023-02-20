use std::cmp::Reverse;
use std::collections::BinaryHeap;
pub fn max_points(mut grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut qs = vec![];
    for (i, &q) in queries.iter().enumerate() {
        qs.push((q, i));
    }

    qs.sort();
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(grid[0][0]), 0, 0));
    grid[0][0] = 0;
    let mut cnt = 0;
    let mut ret = vec![0; qs.len()];
    let (n, m) = (grid.len(), grid[0].len());

    for q in qs {
        let (limit, idx) = (q.0, q.1);
        // println!("{:?}", &pq);
        while !pq.is_empty() && pq.peek().unwrap().0 .0 < limit {
            let item = pq.pop().unwrap();
            let (x, y) = (item.1, item.2);
            cnt += 1;
            for (xx, yy) in [(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)] {
                if xx >= 0 && xx < n && yy >= 0 && yy < m && grid[xx][yy] > 0 {
                    pq.push((Reverse(grid[xx][yy]), xx, yy));
                    grid[xx][yy] = 0;
                }
            }
        }
        ret[idx] = cnt;
    }
    ret
}
