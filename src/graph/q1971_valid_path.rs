struct UF {
    p: Vec<usize>,
}

impl UF {
    fn new(n: usize) -> Self {
        UF {
            p: (0..=n).collect(),
        }
    }

    fn find(&mut self, u: usize) -> usize {
        if self.p[u] == u {
            return u;
        } else {
            let pu = self.find(self.p[u]);
            self.p[u] = pu;
            return self.p[u];
        }
    }

    fn union(&mut self, u: usize, v: usize) {
        if self.p[u] == self.p[v] {
            return;
        } else {
            let (pu, pv) = (self.find(self.p[u]), self.find(self.p[v]));
            self.p[pu] = pv;
        }
    }
}

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let n = n as usize;
    let mut UF = UF::new(n);
    for e in edges {
        UF.union(e[0] as usize, e[1] as usize);
    }
    UF.find(source as usize) == UF.find(destination as usize)
}
