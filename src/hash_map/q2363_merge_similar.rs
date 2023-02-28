pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::BTreeMap;
    let mut cache = BTreeMap::new();
    items1
        .iter()
        .chain(items2.iter())
        .for_each(|v| *cache.entry(v[0]).or_insert(0) += v[1]);
    cache.iter().map(|(&k, &v)| vec![k, v]).collect::<_>()
}
