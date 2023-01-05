const NUM_LEN: usize = 15;
#[derive(Default)]
struct Trie {
    cnt: i32,
    child: [Option<Box<Trie>>; 2]
}
impl Trie {
    fn add(&mut self, num: i32) {
        let mut root = self;
        for i in (0..=NUM_LEN).rev() {
            let bit = ((num >> i) & 1) as usize;
            root = root.child[bit].get_or_insert(Box::new(Trie::default()));
            root.cnt += 1;
        }
    }

    fn query(&self, num: i32, bound: i32) -> i32 {
        let mut root = self;
        let mut ret = 0;
        for i in (0..=NUM_LEN).rev() {
            let (num_bit, bound_bit)= (((num >> i) & 1) as usize, ((bound >> i) & 1) as usize);
            if bound_bit == 1 {
                // num_bit ^ pre_bit = 0 < bound_bit
                if let Some(l) = root.child[num_bit].as_ref() {
                    ret += l.cnt;
                }
                // num_bit ^ pre_bit = 1 = bound_bit
                if let Some(r) = root.child[num_bit ^ 1].as_ref() {
                    root = r;
                } else {
                    return ret;
                }
            } else {
                // num_bit ^ pre_bit = 0 < bound_bit
                if let Some(l) = root.child[num_bit].as_ref() {
                    root = l;
                } else {
                    return ret;
                }
            }
        }
        ret += root.cnt;
        ret
    }
}

pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
    let mut root = Trie::default();
    let mut ret = 0;
    for num in nums {
        ret += root.query(num, high) - root.query(num, low - 1);
        root.add(num);
    }
    ret
}
