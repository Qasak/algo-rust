// naive dfs
pub fn closest_cost(mut arr1: Vec<i32>, mut arr2: Vec<i32>, target: i32) -> i32 {
    let (n, m) = (arr1.len(), arr2.len());
    arr1.sort(); arr2.sort();
    arr1.dedup();
    let mut cache = vec![];
    fn dfs(i: usize, arr2: &Vec<i32>, cache: &mut Vec<i32>, path_sum: i32, target: i32) {
        if i == arr2.len() {
            cache.push(path_sum);
            return;
        }
        dfs(i + 1, arr2, cache, path_sum + 2 * arr2[i], target);
        dfs(i + 1, arr2, cache, path_sum + arr2[i], target);
        dfs(i + 1, arr2, cache, path_sum, target);
    }
    dfs(0, &arr2, &mut cache, 0, target);
    cache.sort();
    let (mut cur, mut diff, mut arr3, mut ret) = (0, i32::MAX, vec![], 0);
    for i in arr1 {
        cur = i;
        for &j in &cache {
            cur += j;
            arr3.push(cur);
            cur -= j;
        }
    }
    arr3.sort(); arr3.dedup();
    for i in arr3 {
        if (i - target).abs() < diff {
            ret = i;
            diff = (i - target).abs();
        }
    }
    ret
}