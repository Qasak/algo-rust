use crate::sys::s6_ds_ptr_4_customized_ptr::MyString::Standard;
// use regex::internal::Input;
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::{ops::Deref, str};

const MINI_STRING_MAX_LEN: usize = 30;
struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    // new 接口不暴露出去，保证传入的 v 的字节长度小于等于 30
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
        // unsafe { str::from_utf8_unchecked(&self.data[..self.len as usize])
    }
}

impl Debug for MiniString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl MyString {
    pub fn push_str(&mut self, string: &str) {
        match self {
            // ref用来匹配模式，默认的匹配需要移动，而加了ref就不用
            MyString::Inline(m) => {
                let len = m.len as usize;
                let new_len = string.len() + len;
                println!("{:?}", (len, string.len(), new_len));
                if new_len <= MINI_STRING_MAX_LEN {
                    m.data[len..new_len].copy_from_slice(string.as_bytes());
                    m.len = new_len as u8;
                } else {
                    *self = Standard(string.to_owned());
                }
            }
            MyString::Standard(s) => s.push_str(string),
        }
    }
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            // ref用来匹配模式，默认的匹配需要移动，而加了ref就不用
            MyString::Inline(v) => v.deref(),
            MyString::Standard(v) => v.deref(),
        }
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        match s.len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s.to_owned()),
            _ => Self::Inline(MiniString::new(s)),
        }
    }
}

impl From<String> for MyString {
    fn from(s: String) -> Self {
        match s.len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s),
            _ => Self::Inline(MiniString::new(s)),
        }
    }
}

impl Display for MyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[test]
fn f() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("{:?}", (len1, len2));

    let s1: MyString = "short".into();
    let s2: MyString = "如果我没记错的话，秘籍应该是⬆️⬆️⬇️⬇️⬅️➡️⬅️➡️ABAB".into();
    //debug 输出
    println!("{:?}, {:?}", s1, s2);
    //display输出
    println!(
        "s1{}({} bytes, {} chars), s2:{}({} bytes, {} chars)",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );
    // MyString 可以使用一切 &str 接口，因为 Rust 可以自动 Deref
    assert!(s1.ends_with("rt"));
    assert!(s2.ends_with("ABAB"));
}

// 支持从 String 中生成一个 MyString
#[test]
fn q1() {
    let s1: MyString = format!("{} tf ?", "what").into();
    println!("{}, {}", s1, s1.len());
}

// 加上类似 String 的 push_str 接口
#[test]
fn q2() {
    let mut s1: MyString = "what tf ?".into();
    s1.push_str("🌍");
    println!("{}", s1);
}

// Cow<[u8]> 和 Cow<str> 的大小
// Cow<'a, B> 要求 B 实现 ToOwned，
// 其Owned变体的数据为 对应的 Owned 类型，即 [T] 对应的是 Vec<T>， str 对应的是 String，
// 这两个的大小都是24字节，加上枚举占用的一字节以及8字节对齐，就是32字节。
#[test]
fn q3() {
    let len1 = std::mem::size_of::<Cow<[u8]>>();
    let len2 = std::mem::size_of::<Cow<str>>();

    println!("{}", len1);
    println!("{}", len2);
}
