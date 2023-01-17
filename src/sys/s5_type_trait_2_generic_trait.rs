use std::str::FromStr;
use regex::Regex;
pub trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error> where Self: Sized;
}

// impl Parse for u8 {
//     fn parse(s: &str) -> Self {
//         let re: Regex = Regex::new(r"\d+").unwrap();
//         if let Some(captures) = re.captures(s) {
//             // 取第一个 match，将其捕获的 digits 换成 u8
//             captures
//                 .get(0)
//                 .map_or(0, |s| s.as_str().parse().unwrap_or(0))
//         } else {
//             0
//         }
//     }
// }
//
// impl Parse for f64 {
//     fn parse(s: &str) -> Self {
//         let re: Regex = Regex::new(r"\d+\.\d+").unwrap();
//         if let Some(captures) = re.captures(s) {
//             // 取第一个 match，将其捕获的 digits 换成 u8
//             captures
//                 .get(0)
//                 .map_or(0.0, |s| s.as_str().parse().unwrap_or(0.0))
//         } else {
//             0.0
//         }
//     }
// }

// 我们约束 T 必须同时实现了 FromStr 和 Default
// 这样在使用的时候我们就可以用这两个 trait 的方法了
impl<T> Parse for T
    where
        T: FromStr
{
    type Error = String;

    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(r"\d+(\.\d+)?").unwrap();
        // captures: 匹配最左的第一个
        if let Some(captures) = re.captures(s) {
            println!("{:?}", captures);
            captures
                // get: 没有匹配到第i组时返回None
                .get(0)
                // 当没匹配上，返回None时，返回自定义的Error；否则拿到Some里面的Match，并apply一个函数
                // 实际上这里永远不会触发，没匹配上只会触发最外层，除非把get(0)改成get(1)让他报错
                .map_or(Err("🔥".to_string()), |s| {
                    s.as_str()
                        // 把Match转成&str, 把&str parse成T，这里就用到了T: FromStr
                        .parse()
                        // &str转T失败，返回自定义错误
                        .map_err(|_err| "😡".to_string())
                })
        } else {
            // 没有匹配时返回None
            Err("💣".to_string())
        }
    }
}

mod test {
    use crate::sys::s5_type_trait_2_generic_trait::Parse;
    use regex::Regex;

    #[test]
    fn parse_should_work() {
        // 没匹配上
        assert_eq!(u32::parse("abcd"), Err("💣".into()) );
        // parse出错
        assert_eq!(u8::parse("abcd257"), Err("😡".into()) );
        assert_eq!(f64::parse("xxxx xx114.514 hello world 256"), Ok(114.514));
    }

    #[test]
    fn f () {

        println!("result: {}", u8::parse("11 255 hello world 256").unwrap());
        println!("result: {}", f64::parse("111234 255.1234 hello world 256").unwrap());
        println!("result: {}", f64::parse("aaa111").unwrap());

    }

}
