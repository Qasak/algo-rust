use rand::Rng;

pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = n - k as usize;
    quickselect(&mut nums, k)
}

fn quickselect<T: Ord + Copy>(arr: &mut [T], k: usize) -> T {
    let len = arr.len();
    let pivot = partition(arr);
    match pivot.cmp(&k) {
        std::cmp::Ordering::Equal => arr[pivot],
        std::cmp::Ordering::Greater => quickselect(&mut arr[0..pivot], k),
        std::cmp::Ordering::Less => quickselect(&mut arr[pivot + 1..len], k - pivot - 1),
    }
}

fn partition<T: Ord + Copy>(arr: &mut [T]) -> usize {
    let mut rng = rand::thread_rng();
    let len = arr.len();
    let pivot_index = rng.gen_range(0, len);
    arr.swap(pivot_index, len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

#[cfg(test)]
mod test {
    use std::{rc::Rc, sync::Arc};

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
