pub fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
    let mut coins = coins;
    coins.sort_unstable();
    let mut ans = 0;
    for c in coins {
        if c > ans + 1 {
            break;
        }
        ans += c;
    }
    ans + 1
}