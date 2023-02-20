// UnionFind
struct UF {
    p: Vec<usize>,
}

impl UF {
    fn new(n: usize) -> Self {
        UF {
            p: (0..=n).collect(),
        }
    }

    fn union(&mut self, u: usize, v: usize) {
        if self.p[u] != self.p[v] {
            let pu = self.find(u);
            let pv = self.find(v);
            self.p[pu] = pv;
        }
    }

    fn find(&mut self, u: usize) -> usize {
        if self.p[u] == u {
            u
        } else {
            let pu = self.p[u];
            let ppu = self.find(pu);
            self.p[u] = ppu;
            self.p[u]
        }
    }
}

pub fn distance_limited_paths_exist(
    n: i32,
    mut edge_list: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    let mut qs = vec![];
    for (i, q) in queries.iter().enumerate() {
        qs.push((q[0] as usize, q[1] as usize, q[2] as usize, i));
    }
    qs.sort_by(|a, b| a.2.cmp(&b.2));
    edge_list.sort_by(|a, b| a[2].cmp(&b[2]));

    let (nv, ne, nq) = (n as usize, edge_list.len(), queries.len());
    let mut uf = UF::new(nv);
    let mut ret = vec![false; nq];
    let mut j = 0;

    for i in 0..nq {
        let (u, v, limit, idx) = (qs[i].0, qs[i].1, qs[i].2, qs[i].3);
        while j < ne {
            let (uu, vv, w) = (
                edge_list[j][0] as usize,
                edge_list[j][1] as usize,
                edge_list[j][2] as usize,
            );
            if w >= limit {
                break;
            }
            uf.union(uu, vv);
            j += 1;
        }
        ret[idx] = uf.find(u) == uf.find(v);
    }
    ret
}
