use std::cmp::Reverse;
use std::collections::HashMap;

// direct sort solution
pub fn frequency_sort(s: String) -> String {
    let map: HashMap<char, u32> = s.chars().fold(HashMap::new(), |mut m, c| {
        *m.entry(c).or_default() += 1;
        m
    });
    let mut cs: Vec<char> = s.chars().collect();
    cs.sort_by_key(|&c| (Reverse(map[&c]), c));
    cs.into_iter().collect::<String>()
}

// bucket
pub fn frequency_sort_1(s: String) -> String {
    let map: HashMap<char, usize> = s.chars().fold(HashMap::new(), |mut m, c| {
        *m.entry(c).or_default() += 1;
        m
    });

    // put to bucket
    let mut bucket: Vec<String> = vec!["".to_string(); s.len() + 1];
    for (c, cnt) in map {
        bucket[cnt].push_str(&c.to_string().repeat(cnt));
    }

    // convert bucket to output, from most frequent to least frequent chars
    let mut result = "".to_string();
    for s in bucket.iter().rev() {
        result.push_str(s);
    }

    result
}
