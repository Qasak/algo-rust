pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    (0..n - 2)
        .map(|i| (0..n - 2).map(|j| get_max(i, j, &grid)).collect())
        .collect()
}

fn get_max(i: usize, j: usize, grid: &[Vec<i32>]) -> i32 {
    (i..(i + 3))
        .flat_map(|ii| (j..(j + 3)).map(move |jj| grid[ii][jj]))
        .max()
        .unwrap()
}

pub fn largest_local_1(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let mut ret = vec![vec![0; n - 2]; n - 2];
    (0..(n - 2)).for_each(|i| {
        (0..(n - 2)).for_each(|j| {
            ret[i][j] = get_max_1(i, j, &grid);
        })
    });
    ret
}

fn get_max_1(i: usize, j: usize, grid: &[Vec<i32>]) -> i32 {
    let mut ret = 0;
    (i..(i + 3)).for_each(|ii| (j..(j + 3)).for_each(|jj| ret = ret.max(grid[ii][jj])));
    ret
}
