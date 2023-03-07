use std::collections::HashSet;

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut line = vec![];
    let mut vis = HashSet::new();
    dfs(&mut ret, &mut line, &mut vis, &nums, 0);
    ret
}

fn dfs(ret: &mut Vec<Vec<i32>>, line: &mut Vec<i32>, vis: &mut HashSet<i32>, nums: &Vec<i32>, i: usize) {
    if i == nums.len() {
        ret.push(line.clone());
        return;
    }
    for j in nums {
        if vis.contains(j) {
            continue;
        }
        vis.insert(*j);
        line.push(*j);
        dfs(ret, line, vis, nums, i + 1);
        line.pop();
        vis.remove(j);
    }
}

pub fn permute_1(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = (2..=nums.len()).product();
    let mut res = Vec::with_capacity(len);

    dfs_1(&mut nums, 0, &mut res);

    res
}
pub fn dfs_1(v: &mut Vec<i32>, idx: usize, res: &mut Vec<Vec<i32>>) {
    if idx == v.len() {
        res.push(v.clone());
        return;
    }

    for i in idx..v.len() {
        v.swap(idx, i);
        dfs_1(v, idx + 1, res);
        v.swap(idx, i);
    }
}