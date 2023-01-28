use std::str::FromStr;
use regex::Regex;
use std::ops::Add;

// 为{具体类型}实现trait
#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

// 对 Complex 类型的实现
impl Add for Complex {
    type Output = Self;

    // 注意 add 第一个参数是 self，会移动所有权
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}

// ...

// 如果不想移动所有权，可以为 &Complex 实现 add，这样可以做 &c1 + &c2
impl Add for &Complex {
    // 注意返回值不应该是 Self 了，因为此时 Self 是 &Complex
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

// ...

// 因为 Add<Rhs = Self> 是个泛型 trait，我们可以为 Complex 实现 Add<f64>
impl Add<f64> for &Complex {
    type Output = Complex;

    // rhs 现在是 f64 了
    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;
        Complex::new(real, self.imagine)
    }
}

#[test]
fn test_complex() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);
    println!("{:?}", &c1 + &c2);
    println!("{:?}", &c1 + 5.0);
    println!("{:?}", c1 + c2);
}

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


// 为{泛型}实现trait，这个泛型要满足某一类约束
// str::parse 是一个泛型函数，它返回任何实现了 FromStr trait 的类型，它必须实现了 FromStr trait。

// 两种约束形式甚至可以一起写
impl<T: FromStr> Parse for T
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
                        // map_err() 传一个处理错误的函数
                        .map_err(|_err| "😡".to_string())
                })
        } else {
            // 没有匹配时返回None
            Err("💣".to_string())
        }
    }
}

mod test {
    use regex::Regex;
    use crate::sys::s5_type_trait_2_generic_trait::Parse;

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
