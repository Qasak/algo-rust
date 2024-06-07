use std::collections::HashMap;

struct Bitree {
    tree: Vec<i32>
}

impl Bitree {
    fn new(n: usize) -> Self {
        let mut tree = vec![0; n + 1];
        Self {
            tree
        }
    }

    fn add(&mut self, mut index: usize) {
        while index < self.tree.len() {
            self.tree[index] += 1;
            index += index & (!index + 1);
        }
    }

    fn pre(&self, mut index: usize) -> i32 {
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= index & (!index + 1);
        }
        sum
    }
}


impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let lower = lower as i64;
        let upper = upper as i64;
        // 区间和->想到前缀和

        // 枚举前缀和数组的右端点j
        // 使得lower <= s[j]-s[i] <= upper。可得s[j] - upper <= s[i] <= s[j] - lower。
        // 即
        // 满足在[lower, upper]的区间和的个数转换为区间[s[j] - upper,s[j] - lower]中存在的s[i]的个数
        // 
        // 把前缀和的值，对应端点的值映射为下标

        let n = nums.len();
        let mut p: Vec<i64> = vec![0; n + 1];
        for i in 0..n {
            p[i + 1] = p[i] + nums[i] as i64;
        }
        // 把前缀和的值，对应端点的值映射为下标
        let mut p_sort = p.clone();
        for &s in p.iter() {
            p_sort.push(s - upper);
            p_sort.push(s - lower);
        }
        p_sort.sort();
        p_sort.dedup();
        let mut index = HashMap::new();
        for i in 0..p_sort.len() {
            index.insert(p_sort[i], i + 1);
        }
        let m = p_sort.len();
        // println!("{:?}", p);
        //遍历s[j]，统计对应的s[i]个数
        let mut bitree = Bitree::new(m);
        // bitree.add(index[&p[0]]);
        let mut ret = 0;
        for i in 0..=n {
            // l = s[i] - upper, r = s[i] - lower
            let (l, r) = (index[&(p[i] - upper)], index[&(p[i] - lower)]);
            ret += bitree.pre(r) - bitree.pre(l - 1);
            bitree.add(index[&p[i]]);
        }
        ret
    }
}