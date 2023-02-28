fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
    let g = (0..(1 << n)).map(|i| i ^ (i >> 1)).collect::<Vec<_>>();
    let j = g.iter().position(|&x| x == start).unwrap();
    [&g[j..], &g[..j]].concat()
}
