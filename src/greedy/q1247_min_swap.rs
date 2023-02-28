pub fn minimum_swap(s1: String, s2: String) -> i32 {
    let (xy, yx) = s1.chars().zip(s2.chars()).fold((0, 0), |mut acc, (s, t)| {
        match (s, t) {
            ('x', 'y') => acc.0 += 1,
            ('y', 'x') => acc.1 += 1,
            _ => {}
        }
        acc
    });

    match (xy + yx) % 2 {
        1 => -1,
        _ => xy / 2 + yx / 2 + xy % 2 + yx % 2,
    }
}
