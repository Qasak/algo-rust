use std::{str, ops::Deref};
use std::fmt::{Debug, Display, Formatter};

const MINI_STRING_MAX_LEN: usize = 30;
struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN]
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
    Standard(String)
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref()
        }
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        match s.len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s.to_owned()),
            _ => Self::Inline(MiniString::new(s))
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