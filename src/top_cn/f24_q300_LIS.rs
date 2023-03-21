// naive LIS
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut f = vec![1; n];
    for i in 1..n {
        for j in 0..i {
            if nums[i] > nums[j] {
                f[i] = f[i].max(f[j] + 1);
            }
        }
    }
    *f.iter().max().unwrap()
}

// greedy: O(nlogn) time, O(n) space.
// 1. if x is larger than all tails, append it, increase the size by 1
// 2. if tails[i-1] < x <= tails[i], update tails[i]
// 3. if x is smaller than all tails, update tails[0]
// 4. return the size
pub fn length_of_lis_greedy(nums: Vec<i32>) -> i32 {
    let mut f = vec![];
    for &n in &nums {
        if let Err(i) = f.binary_search(&n) {
            if i == f.len() {
                f.push(n);
            } else {
                f[i] = n;
            }
        }
    }
    f.len() as i32
}

#[test]
fn test_length_of_lis_greedy() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(length_of_lis_greedy(nums), 4);
}

#[test]
fn test_bs() {
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    assert_eq!(s.binary_search(&13), Ok(9));
    assert_eq!(s.binary_search(&4), Err(7));
    assert_eq!(s.binary_search(&100), Err(13));
    let r = s.binary_search(&1);

    eprintln!("r.unwrap() = {:?}", r.unwrap());
}
