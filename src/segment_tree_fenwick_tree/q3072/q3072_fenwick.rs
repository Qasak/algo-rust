use std::collections::HashMap;
struct BinaryIndexedTree {
    tree: Vec<i32>,
}
impl BinaryIndexedTree {
    // 给你一个下标从 1 开始、长度为 n 的整数数组 nums 。
    // 提示这是个线段树的题，下标从 1 开始
    // 严格大于：n-小于等于
    fn new(n: usize) -> Self {
        BinaryIndexedTree {
            tree: vec![0; n + 1],
        }
    }
    // # 把下标为 i 的元素个数增加 1
    // 更新：从子到根
    fn add(&mut self, mut i: usize) {
        while i < self.tree.len() {
            self.tree[i] += 1;
            // 注意 -i = !i + 1
            i += i & (!i + 1);
        }
    }
    // # 返回下标在 [1,i] 的元素个数之和
    // 查询：从特定点到1
    fn get(&self, mut i: usize) -> i32 {
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & (!i + 1);
        }
        sum
    }
}

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        // 2棵树状数组分别维护 a 和 b 中的每个元素的出现次数
        let n = nums.len();
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        let mut index = HashMap::new();
        for i in 0..n {
            index.insert(sorted_nums[i], i + 1);
        }

        let mut arr1 = vec![nums[0]];
        let mut arr2 = vec![nums[1]];
        // 用n初始化，因为会出现查询当前满足条件元素==0的情况
        let mut tree1 = BinaryIndexedTree::new(n);
        let mut tree2 = BinaryIndexedTree::new(n);
        tree1.add(index[&nums[0]]);
        tree2.add(index[&nums[1]]);

        for i in 2..n {
            let count1 = arr1.len() as i32 - tree1.get(index[&nums[i]]);
            let count2 = arr2.len() as i32 - tree2.get(index[&nums[i]]);
            if count1 > count2 || (count1 == count2 && arr1.len() <= arr2.len()) {
                arr1.push(nums[i]);
                tree1.add(index[&nums[i]]);
            } else {
                arr2.push(nums[i]);
                tree2.add(index[&nums[i]]);
            }
        }
        arr1.extend(arr2);
        arr1
    }
}