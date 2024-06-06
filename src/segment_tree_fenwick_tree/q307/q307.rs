struct NumArray {
    nums: Vec<i32>,
    tree: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        // 原数组无需扩展长度，fenwick数组增加1长度
        let n = nums.len();
        let mut tree = vec![0; n + 1];
        let mut num_array = NumArray {
            nums,
            tree
        };
        for i in 0..n {
            let mut index = i + 1;
            while index < num_array.tree.len() {
                num_array.tree[index] += num_array.nums[i];
                index += index & (!index + 1);
            }
        }
        num_array
    }
    
    // &self -> &mut self
    fn update(&mut self, index: i32, val: i32) {
        let mut index = index as usize;
        let delta = val - self.nums[index];
        self.nums[index] = val;
        index += 1;
        while index < self.tree.len() {
            self.tree[index] += delta;
            index += index & (!index + 1);
        }

    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.pre(right + 1) - self.pre(left)
    }

    fn pre(&self, index: i32) -> i32 {
        let mut index = index as usize;
        let mut sum = 0;
        while index > 0 {
            sum += self.tree[index];
            index -= index & (!index + 1);
        }
        sum
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */