use rand::Rng;

// heap solution
pub fn find_kth_largest_heap(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BinaryHeap;
    let mut q = BinaryHeap::new();
    nums.iter().for_each(|&i| q.push(i));
    (1..k).for_each(|_| {
        q.pop();
    });
    *q.peek().unwrap()
}


pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    fn qss(arr: &mut Vec<i32>, l: usize, r: usize, k: usize) {
        if l >= r {
            return;
        }
        let (le, eq) = partition(arr, l, r);
        // usize overflow
        if k as i32 <= (le as i32 - 1) {
            qss(arr, l, le - 1, k);
        }
        if k >= eq + 1 {
            qss(arr, eq + 1, r, k);
        }
    }
    fn partition(arr: &mut Vec<i32>, l: usize, r: usize) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        // if use rand 0.8.5
        // let pick = rng.gen_range(l..r);
        let pick = rng.gen_range(l, r + 1);
        let pivot = arr[pick];
        swap(arr, pick, r);
        // le: next pos of less/ first item equal to pivot
        // eq: next pos of eq/ first item greater to pivot
        // gt: left next pos of gt/ last item equal to pivot
        let (mut le, mut eq, mut gt) = (l, l, r - 1);
        while gt > 0 && eq <= gt {
            if arr[eq] == pivot {
                eq += 1;
            } else if arr[eq] < pivot {
                swap(arr, eq, le);
                eq += 1;
                le += 1;
            } else {
                swap(arr, eq, gt);
                // usize overflow
                if gt == 0 {
                    break;
                }
                gt -= 1;
            }
        }

        swap(arr, eq, r);
        (le, eq)
    }
    fn swap(arr: &mut Vec<i32>, i: usize, j: usize) {
        let t = arr[i];
        arr[i] = arr[j];
        arr[j] = t;
    }
    let k = k as usize;
    let n = nums.len();
    qss(&mut nums, 0, n - 1, n - k);
    nums[n - k]
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use crate::top_cn::f4_q215_kth_largest_elem_in_array::find_kth_largest;

    #[test]
    fn f() {
        for i in 0..20 {
            println!(
                "{}",
                find_kth_largest(
                    [
                        3, 2, 13, 2, 3, 3, 2, 13, 2, 3, 1, 2, 4, 5, 5, 6, 5, 6, 43, 2, 13, 2, 3, 1,
                        2, 4, 5, 5, 6, 5, 6, 41, 2, 4, 5, 5, 6, 5, 6, 4
                    ]
                    .to_vec(),
                    11
                )
            );
        }
        for i in 0..20 {
            println!(
                "{}",
                find_kth_largest(
                    [
                        3, 2, 13, 2, 3, 1, 2, 4, 53, 2, 13, 2, 3, 3, 2, 13, 2, 3, 1, 2, 4, 5, 5, 6,
                        5, 6, 43, 2, 13, 2, 3, 1, 2, 4, 5, 5, 6, 5, 6, 41, 2, 4, 5, 5, 6, 5, 6, 43,
                        2, 13, 2, 3, 3, 2, 13, 2, 3, 1, 2, 4, 5, 5, 6, 5, 6, 43, 2, 13, 2, 3, 1, 2,
                        4, 5, 5, 6, 5, 6, 41, 2, 4, 5, 5, 6, 5, 6, 4, 5, 6, 5, 6, 4
                    ]
                    .to_vec(),
                    1
                )
            );
        }
        for i in 0..20 {
            println!("{}", find_kth_largest([2, 1].to_vec(), 1));
        }
    }
}
