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
        let n = nums.len();
        let mut tree = vec![0; n + 1];
        let mut a = vec![0; n + 1];

        for i in 0..n {
            a[i + 1] = nums[i];
        }
        let mut num_array = NumArray {
            nums: a,
            tree: tree
        };
        for i in 0..n {
            let mut index = i + 1;
            while index < num_array.nums.len() {
                num_array.tree[index] += nums[i];
                index += index & (!index + 1);
            }
        }
        num_array
    }
    
    // &self -> &mut self
    fn update(&mut self, index: i32, val: i32) {
        let mut index = index as usize + 1;
        let delta = val - self.nums[index];
        self.nums[index] = val;
        while index < self.nums.len() {
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