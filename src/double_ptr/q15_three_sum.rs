pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut arr = nums;
    let mut ret = vec![];
    arr.sort();
    let (mut i, n) = (0, arr.len());
    while i < n {
        let (mut l, mut r, target) = (i + 1, n - 1, -arr[i]);
        while l < r {
            if arr[l] + arr[r] == target {
                ret.push(vec![arr[i], arr[l], arr[r]]);
                l += 1; r -= 1;
                while l < r && arr[l] == arr[l - 1] {l += 1;}
                while l < r && arr[r] == arr[r + 1] {r -= 1;}
            } else if arr[l] + arr[r] < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
        i += 1;
        while i < n && arr[i] == arr[i - 1] {i += 1;}
    }
    ret
}

#[cfg(test)]
mod test {
    use crate::double_ptr::q15_three_sum::three_sum;

    #[test]
    fn f() {
        println!("{:?}", three_sum(vec![-4,-3,-2,1,4,-51,22,32,0,52]));
    }
}