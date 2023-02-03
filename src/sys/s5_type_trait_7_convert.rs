

struct MyString(String);
impl From<MyString> for i32 {
    fn from(value: MyString) -> Self {
        let mut ret = 0;
        let mut iter = value.0.bytes();
        while let Some(b) = iter.next() {
            ret *= 10;
            ret += (b - b'0') as i32;
        }
        ret
    }
}

impl From<i32> for MyString {
    fn from(value: i32) -> Self {
        MyString(value.to_string())
    }
}

#[test]
fn convert_test() {
    let x = i32::from(MyString("456".to_string()));
    println!("{}", x);
    let s: MyString = x.into();

    // MyString自动实现Into
    let y: i32 = s.into();
    println!("{}", y);
}


use std::fmt;
use std::fs::File;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn print(v: impl Into<IpAddr>) {
    println!("{:?}", v.into());
}

#[test]
fn test_ip_convert() {
    let v4: Ipv4Addr = "2.2.2.2".to_string().parse().unwrap();
    if let Ok(ip) = "2001:db8:85a3:0::8a2E:0370:7334".to_string().parse::<Ipv6Addr>() {
        println!("{}", ip.to_string());
    }
    let v6: Ipv6Addr = "::1".parse().unwrap();

    // IPAddr 实现了 From<[u8; 4]，转换 IPv4 地址
    print([1, 1, 1, 1]);
    // IPAddr 实现了 From<[u16; 8]，转换 IPv6 地址
    print([0xfe80, 0, 0, 0, 0xaede, 0x48ff, 0xfe00, 0x1122]);
    // IPAddr 实现了 From<Ipv4Addr>
    print(v4);
    // IPAddr 实现了 From<Ipv6Addr>
    print(v6);
}



///# Examples
///```no_run
/// pub fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
///     OpenOptions::new().read(true).open(path.as_ref())
/// }
///
/// impl AsRef<Path> for String {
///     #[inline]
///     fn as_ref(&self) -> &Path {
///         Path::new(self)
///     }
/// }
/// ```
///
#[test]
fn test_open_ref()  {
    use std::fs::File;
    if let Ok(f) = File::open("/tmp/test.txt".to_string()) {
        let mut bytes_iter = f.bytes();
        while let Some(b) = bytes_iter.next() {
            if let Ok(b) = b {
                print!("{}", b as char);
            }
        }
    }
}

use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

// enum 不能 derive Default
#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Elixir => "Elixir",
            Language::Haskell => "Haskell",
        }
    }
}

// impl Display for Language {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "({})", self.as_ref())
//     }
// }

fn print_ref(v: impl AsRef<str>) {
    println!("{}", v.as_ref());
}

#[test]
fn test_enum_ref()  {
    let lang = Language::Rust;
    // &str 实现了 AsRef<str>
    print_ref("Hello world!");
    // String 实现了 AsRef<str>
    print_ref("Hello world!".to_string());
    // 我们自己定义的 enum 也实现了 AsRef<str>
    println!("{:?}", lang);
    print_ref(lang);
}

#[test]
fn test_deref_rc()  {
    let a = Rc::new(1);
    let b = a.clone();
    let y = b.deref();
    let x = *b;
    println!("{}", *b);
    println!("{}", b.deref());
}


#[derive(Debug)]
struct Buffer<T>(Vec<T>);

impl<T> Buffer<T> {
    pub fn new(v: impl Into<Vec<T>>) -> Self {
        Self(v.into())
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[test]
fn test_my_deref() {
    let mut buf = Buffer::new([1, 3, 2, 4]);
    // 因为实现了 Deref 和 DerefMut，这里 buf 可以直接访问 Vec<T> 的方法
    // 下面这句相当于：(&mut buf).deref_mut().sort()，也就是 (&mut buf.0).sort()
    buf.sort();
    println!("buf: {:?}", buf);
}




// Debug Display Default all

// struct 可以 derive Default，但我们需要所有字段都实现了 Default
#[derive(Clone, Debug, Default)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}



// 手工实现 Default
impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}

impl Developer {
    pub fn new(name: &str) -> Self {
        // 用 ..Default::default() 为剩余字段使用缺省值
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}

impl fmt::Display for Developer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ddddisplay!: {}({} years old): {:?} developer",
            self.name, self.age, self.lang
        )
    }
}

#[test]
fn debug_display_default_test() {
    // 使用 T::default()
    let dev1 = Developer::default();
    // 使用 Default::default()，但此时类型无法通过上下文推断，需要提供类型
    let dev2: Developer = Default::default();
    // 使用 T::new
    let mut dev3 = Developer::new("Jas");
    dev3.age = 100;
    println!("dev1: {}\nndev2: {}\nndev3: {:?}", dev1, dev2, dev3);
}