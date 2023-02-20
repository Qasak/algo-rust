use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let (mut d, mut pq, mut g) = (vec![i32::MAX; n + 1], BinaryHeap::new(), HashMap::new());

    for e in edges {
        let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
        g.entry(u).or_insert(HashMap::new()).insert(v, w);
        g.entry(v).or_insert(HashMap::new()).insert(u, w);
    }

    d[n] = 0;
    pq.push((Reverse(d[n]), n));
    while let Some((du, u)) = pq.pop() {
        let du = du.0;
        for (&v, _) in g[&u].iter() {
            let w = g[&u][&v];
            if du + w < d[v] {
                d[v] = du + w;
                pq.push((Reverse(d[v]), v));
            }
        }
    }

    let (mo, mut f) = (1e9 as i32 + 7, vec![0 as i32; n + 1]);
    // f: 从1到i的受限路径数
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
    f[n] % mo
}

#[cfg(test)]
mod test {
    use crate::weekly_contest::w231::q1786_count_restricted_paths::count_restricted_paths;

    #[test]
    fn test1() {
        let n = 9;
        // [[i32:3]; 18]
        let edges = [
            [6, 2, 35129],
            [3, 4, 99499],
            [2, 7, 43547],
            [8, 1, 78671],
            [2, 1, 66308],
            [9, 6, 33462],
            [5, 1, 48249],
            [2, 3, 44414],
            [6, 7, 44602],
            [1, 7, 14931],
            [8, 9, 38171],
            [4, 5, 30827],
            [3, 9, 79166],
            [4, 8, 93731],
            [5, 9, 64068],
            [7, 5, 17741],
            [6, 3, 76017],
            [9, 4, 72244],
        ];
        let edges: Vec<Vec<i32>> = edges.into_iter().map(|edge| edge.into()).collect();
        let ret = count_restricted_paths(n, edges);
        assert_eq!(ret, 6);
    }
}
