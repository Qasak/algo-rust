// naive loop
pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
    let mut n = nums.len();
    let mut t = vec![];
    while n > 1 {
        t = vec![0; n / 2];
        let mut i = 0;
        let mut j = 0;
        while i < n {
            if j % 2 == 0 {
                t[j] = nums[i].min(nums[i + 1]);
            } else {
                t[j] = nums[i].max(nums[i + 1]);
            }
            j += 1;
            i += 2;
        }
        nums = t;
        n /= 2;
    }
    nums[0]
}
