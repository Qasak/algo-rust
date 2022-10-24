pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    // when max of l <= min of r
    let mut l = vec![0; n];
    let mut r = vec![i32::MAX; n];
    l[0] = nums[0];
    r[n - 1] = nums[n - 1];
    for i in 1..n {
        if nums[i] > l[i - 1] {
            l[i] = nums[i];
        } else {
            l[i] = l[i - 1];
        }
    }
    for i in (0..(n - 1)).rev() {
        if nums[i] < r[i + 1] {
            r[i] = nums[i];
        } else {
            r[i] = r[i + 1];
        }
    }
    // println!("{:?}", l);
    // println!("{:?}", r);

    let mut idx = 0;
    for i in 1..n {
        if l[i - 1] <= r[i] {
            idx = i;
            break;
        }
    }
    idx as i32
}
