use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g: HashMap<usize, HashMap<usize, i32>> = HashMap::new();
    for e in edges {
        let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
        g.entry(u).or_insert(HashMap::new()).insert(v, w);
        g.entry(v).or_insert(HashMap::new()).insert(u, w);
    }
    let mut d = vec![i32::MAX; n + 1];
    d[n] = 0;
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(d[n]), n));
    while !pq.is_empty() {
        let m = pq.len();
        for _ in 0..m {
            let item = pq.pop().unwrap();
            let (du, u) = (item.0.0, item.1);
            for  (&v, _) in g[&u].iter() {
                let w = g[&u][&v];
                if du + w < d[v] {
                    d[v] = du + w;
                    pq.push((Reverse(d[v]), v));
                }
            }
        }
    }
    let mo = 1e9 as i64 + 7;
    // 从1到i的受限路径数
    let mut f = vec![0 as i64; n + 1];
    f[1] = 1;
    let mut nodes: Vec<usize> = (1..=n).collect();
    nodes.sort_by(|&a, &b| d[b].cmp(&d[a]));
    for u in nodes {
        for (&v, _) in g[&u].iter() {
            if d[v] < d[u] {
                f[v] = (f[u] + f[v]) % mo;
            }
        }
    }
    (f[n] % mo) as i32
}