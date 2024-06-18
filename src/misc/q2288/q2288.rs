impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        let percent = (100 - discount) as f64 / 100.0;
        sentence.split_whitespace()
            // .map() 方法通常与迭代器（Iterator）一起使用
            // map 方法接收一个闭包作为参数，该闭包定义了如何转换每个单词。
            // Rust 的迭代器是懒加载的，无需立即分配内存来存储中间结果
            .map(|w| {
                if w.starts_with("$") {
                    // f64：f64 的 parse 方法支持科学计数法
                    // u64："1e5" 不能直接解析为 u64 类型
                    if let Ok(price) = w[1..].parse::<u64>() {
                        return format!("${:.2}", price as f64 * percent);
                    }
                }
                // 如果单词不以 $ 开头，或者解析价格失败，则直接返回原始单词的字符串表示。
                w.to_string()
            }).collect::<Vec<_>>().join(" ")
    }
}