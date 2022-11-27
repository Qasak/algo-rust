pub fn check(nums: Vec<i32>) -> bool {
    // 2,1,3,4,2,1,3,4
    // 3,4,5,1,2,3,4,5,1,2
    let n = nums.len();
    let mut arr = vec![0; 2 * n];
    for i in 0..(2 * n) {
        arr[i] = nums[i % n];
    }
    let mut ret = false;
    for i in 0..n {
        let mut cur = true;
        for j in (i + 1)..(i + n) {
            if arr[j - 1] > arr[j] {
                cur = false;
                break;
            }
        }
        ret |= cur;
    }
    ret
}