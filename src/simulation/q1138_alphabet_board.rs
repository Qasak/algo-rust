
pub fn alphabet_board_path(target: String) -> String {
    let mut ans = String::new();
    let mut i = 0;
    let mut j = 0;
    for k in 0..target.len() {
        let v = target.as_bytes()[k] - b'a';
        let x = v / 5;
        let y = v % 5;
        while j > y {
            j -= 1;
            ans.push('L');
        }
        while i > x {
            i -= 1;
            ans.push('U');
        }
        while j < y {
            j += 1;
            ans.push('R');
        }
        while i < x {
            i += 1;
            ans.push('D');
        }
        ans.push('!');
    }
    ans
}