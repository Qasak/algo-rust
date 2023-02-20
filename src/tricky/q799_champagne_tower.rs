// Consider the capacity of each cup to be infinite
pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let mut row = vec![poured as f64];
    let query_glass = query_glass as usize;
    let query_row = query_row as usize;
    for i in 1..=query_row {
        let mut next_row = vec![0 as f64; i + 1];
        for j in 0..i {
            let vol = row[j];
            if vol > 1.0 {
                next_row[j] += (vol - 1.0) / 2.0;
                next_row[j + 1] += (vol - 1.0) / 2.0;
            }
        }
        row = next_row;
    }
    row[query_glass].min(1.0)
}
