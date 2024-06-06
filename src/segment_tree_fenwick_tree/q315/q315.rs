
struct Tree{
    tree: Vec<usize>
}

impl Tree {
    fn new(n: usize) -> Self {
        let mut tree = vec![0; n + 1];
        Self {
            tree
        }
    }

    fn remove(&mut self, mut index: usize) {
        while index < self.tree.len() {
            self.tree[index] -= 1;
            index += index & (!index + 1);
        }
    }

    fn add(&mut self, mut index: usize) {
        while index < self.tree.len() {
            self.tree[index] += 1;
            index += index & (!index + 1);
        }
    }


    fn pre(&self, mut index: usize) -> usize {
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= index & (!index + 1);
        }
        sum
    }
}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums_sort = nums.clone();
        nums_sort.sort();
        // 去重
        nums_sort.dedup();


        let mut bit = Tree::new(n);
        for x in nums.iter() {
            let mut index = nums_sort.binary_search(&x).map_or_else(
                |index| index,  // If element is not found, use the insertion index
                |index| index   // If element is found, use the found index
            );
            bit.add(index + 1);
        }
        let mut ret = vec![];
        for x in nums.iter() {
            let mut index = nums_sort.binary_search(&x).map_or_else(
                |index| index,  // If element is not found, use the insertion index
                |index| index   // If element is found, use the found index
            );
            ret.push(bit.pre(index) as i32);
            bit.remove(index + 1);
        }
        ret
    }
}