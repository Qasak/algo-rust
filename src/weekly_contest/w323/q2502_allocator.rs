struct Allocator {
    mem: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Allocator {

    fn new(n: i32) -> Self {
        Allocator{
            mem: vec![0; n as usize]
        }
    }

    fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let (mut i, mut j) = (0, 0);
        let n = self.mem.len();
        while j < n {
            if self.mem[j] == 0 && j - i + 1 == size as usize {
                for k in i..=j {
                    self.mem[k] = m_id;
                }
                return i as i32;
            }
            if self.mem[j] != 0 {
                i = j + 1;
            }
            j += 1;
        }
        -1
    }

    fn free(&mut self, m_id: i32) -> i32 {
        let mut ret = 0;
        for i in 0..self.mem.len() {
            if self.mem[i] == m_id {
                self.mem[i] = 0;
                ret += 1;
            }
        }
        ret
    }
}

/**
 * Your Allocator object will be instantiated and called as such:
 * let obj = Allocator::new(n);
 * let ret_1: i32 = obj.allocate(size, mID);
 * let ret_2: i32 = obj.free(mID);
 */