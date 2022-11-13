use std::collections::HashMap;

pub fn custom_sort_string(order: String, s: String) -> String {
    let mut map = HashMap::new();
    for (i, c) in order.chars().enumerate() {
        map.insert(c, i);
    }
    let mut cs = s.chars().collect::<Vec<char>>();
    cs.sort_unstable_by_key(|item| if map.contains_key(item) {*map.get(item).unwrap()} else {0});
    cs.into_iter().collect::<String>()
}