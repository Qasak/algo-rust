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
        T: FromStr + Default,
{
    type Error = String;

    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        // 生成一个创建缺省值的闭包，这里主要是为了简化后续代码
        // Default::default() 返回的类型根据上下文能推导出来，是 Self
        // 而我们约定了 Self，也就是 T 需要实现 Default trait
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(Err("failed to capture".to_string()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_err| "failed to parse captured string".to_string())
                })
        } else {
            Err("failed to parse string".to_string())
        }
    }
}

mod test {
    use crate::sys::s5_type_trait_2::Parse;

    #[test]
    fn parse_should_work() {
        assert_eq!(u8::parse("123abcd"), Ok(123));
        assert_eq!(f64::parse("111.234 255.1234 hello world 256"), Ok(111.234));
        assert_eq!( u32::parse("123.45abcd"), Err("failed to parse captured string".into()) );
    }

    // #[test]
    // fn f () {
    //     println!("result: {}", u8::parse("11 255 hello world 256"));
    //     println!("result: {}", f64::parse("111234 255.1234 hello world 256"));
    // }

}
