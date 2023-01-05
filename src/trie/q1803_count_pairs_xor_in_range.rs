#[derive(Default)]
struct TrieNode {
    cnt: i32,
    children: [Option<Box<TrieNode>>; 2],
}
impl TrieNode {
    pub fn insert(&mut self, x: i32) {
        let mut temp = self;
        for i in (0..17).rev() {
            let j = ((x >> i) & 1) as usize;
            temp = temp.children[j].get_or_insert(Box::new(TrieNode::default()));
            temp.cnt += 1;
        }
    }
    pub fn query(&self, x: i32, right: i32) -> i32 {
        let mut res = 0;
        let mut temp = self;
        for i in (0..17).rev() {
            let bit_num = ((x >> i) & 1) as usize;
            let bit_lim = (right >> i) & 1;
            if bit_lim == 1 {
                if let Some(next) = temp.children[bit_num].as_ref() {
                    res += next.cnt ;
                }
                if let Some(next) = temp.children[1 - bit_num].as_ref() {
                    temp = next;
                } else {
                    return res;
                }
            } else {
                if let Some(next) = temp.children[bit_num].as_ref() {
                    temp = next;
                } else {
                    return res;
                }
            }
        }
        res += temp.cnt ;
        return res;
    }
}
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        let mut root = TrieNode::default();
        let mut ans = 0;
        for i in nums {
            root.insert(i);
            ans += root.query(i, high) - root.query(i, low - 1);
        }
        ans
    }
}