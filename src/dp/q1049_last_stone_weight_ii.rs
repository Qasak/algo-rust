// define f[w] as `Maximum weight` of selecting any stone among the first i stones
// which weighing no more than the weight of w
pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let tot = stones.iter().sum::<i32>();
    let mut f = vec![0; tot as usize + 1];
    let target = tot as usize / 2 ;
    for s in stones {
        for w in (1..=target).rev() {
            if w < s as usize {continue;}
            f[w] = f[w].max(f[w - s as usize] + s);
        }
    }
    (tot - f[target] - f[target]).abs()
}