pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn bs1(nums: &Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let (mut l, mut r) = (0, n);
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        return if l == n || nums[l] != target {
            -1
        } else {
            l as i32
        }
    }
    fn bs2(nums: &Vec<i32>, target: i32) -> i32 {
        let n = nums.len() as i32;
        let (mut l, mut r) = (-1, n - 1);
        while l < r {
            let m = l + (r - l + 1) / 2;
            if nums[m as usize] > target {
                r = m - 1;
            } else {
                l = m;
            }
        }
        return if l == -1 || nums[l as usize] != target {
            -1
        } else {
            l as i32
        }
    }
    vec![bs1(&nums, target), bs2(&nums, target)]
}