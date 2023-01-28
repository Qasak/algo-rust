pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
    let (mut o_sum, mut e_sum) = (0, 0);
    let n = nums.len();
    for i in 0..n {
        if i % 2 == 0 {
            e_sum += nums[i];
        } else {
            o_sum += nums[i];
        }
    }
    // println!("{:?}", (e_sum, o_sum));
    let mut ret = 0;
    let (mut o_pre, mut e_pre) = (0, 0);

    for i in 0..n {
        if i % 2 == 0 {
            e_pre += nums[i];
        } else {
            o_pre += nums[i];
        }

        // i作为分界线，删除i后，后半段奇数偶数互换
        // [偶数和] == [奇数和] ==
        // [(当前偶数和 - 当前元素) + (后半段奇数和 == (奇数总和 - 前半段奇数和))] == [前半段奇数和 + 后半段偶数和]
        // 反之亦然
        if i % 2 == 0        && (e_pre - nums[i]) + (o_sum - o_pre) == o_pre + (e_sum - e_pre) {
            ret += 1;
        } else if i % 2 == 1 && (o_pre - nums[i]) + (e_sum - e_pre) == e_pre + (o_sum - o_pre) {
            ret += 1;
        }
    }
    ret
}