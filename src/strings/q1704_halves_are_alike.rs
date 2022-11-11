pub fn halves_are_alike(s: String) -> bool {
    let vow = "AEIOUaeiou";
    s.chars().enumerate().
        fold(0, |cnt, (i, c)|
            if vow.contains(c) {
                if i < s.len() / 2 {cnt + 1} else {cnt - 1}
            } else {cnt}
        ) == 0
}