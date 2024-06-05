use std::collections::BTreeMap;
// BTree (TLE)
impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr1 = BTreeMap::new();
        let mut arr2 = BTreeMap::new();
        let mut l1 = Vec::new();
        let mut l2 = Vec::new();
        // 1.entry 方法用于访问或插入一个键值对。如果键 nums[0] 存在于 arr1 中，则返回一个可修改的条目；如果键不存在，则返回一个准备插入的条目。
        // 2.and_modify 方法用于修改一个存在的条目。它接受一个闭包（closure）作为参数，这个闭包会对条目的值进行操作。在这个闭包中，e 是一个可变引用，指向键 nums[0] 对应的值
        // 3.*e += 1 表示将这个值加 1
        // 4..or_insert(1):or_insert 方法用于在键不存在时插入一个新的键值对。如果键 nums[0] 不存在于 arr1 中，则插入该键，并将其值设置为 1。
        arr1.entry(nums[0]).and_modify(|e| *e += 1).or_insert(1);
        arr2.entry(nums[1]).and_modify(|e| *e += 1).or_insert(1);
        l1.push(nums[0]);
        l2.push(nums[1]);
        // 1.iter()是一个对集合中元素的不可变借用：&T
        for &num in nums.iter().skip(2) {
            // 1. arr.range((num + 1)..): 这会返回一个迭代器，包含 arr 中所有键大于 num 的条目。
            // 2..map(|(_, count)| count): 这个映射操作会从每个条目中提取出值（即计数）。(_, count) 是一个模式匹配，其中 _ 表示忽略键，只关心值 count。
            // 3..sum(): 将所有提取出的计数值求和。
            let (l, r): (usize, usize) = (arr1.range((num + 1)..).map(|(_, count)| count).sum(), 
                                          arr2.range((num + 1)..).map(|(_, count)| count).sum());
            if l > r {
                arr1.entry(num).and_modify(|e| *e += 1).or_insert(1);
                l1.push(num);
            } else if l < r {
                arr2.entry(num).and_modify(|e| *e += 1).or_insert(1);
                l2.push(num);
            } else {
                if l1.len() <= l2.len() {
                    arr1.entry(num).and_modify(|e| *e += 1).or_insert(1);
                    l1.push(num);
                } else {
                    arr2.entry(num).and_modify(|e| *e += 1).or_insert(1);
                    l2.push(num);
                }
            }
        }
        // 1.into_iter()将l2转换为一个迭代器，消耗l2
        // 2.extend()将提供的迭代器中的所有元素追加到l1末尾
        l1.extend(l2.into_iter());
        l1
    }
}