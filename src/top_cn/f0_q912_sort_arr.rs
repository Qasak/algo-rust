use rand::Rng;
pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    fn quicksort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        if len < 2 {
            return;
        }
        let pivot = partition(arr);
        quicksort(&mut arr[0..pivot]);
        quicksort(&mut arr[pivot + 1..len]);
    }

    fn partition<T: Ord>(arr: &mut [T]) -> usize {
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
    quicksort(&mut nums);
    nums
}

// merge_sort
// TODO
