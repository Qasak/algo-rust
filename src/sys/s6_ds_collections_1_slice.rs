use std::fmt;

#[test]
fn f() {
    {
        let v = vec![1, 2, 3, 4];
        let v1 = &v;
        let v2 = v.as_slice();
        let v3 = &v[..];
    }
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];
    let s1 = &arr[..2];
    let s2 = &vec[..2];
    println!("s1: {:?}, s2: {:?}", s1, s2);
    // &[T] and &[T] 比较，取决于长度和内容是否相等
    assert_eq!(s1, s2);
    // &[T] and Vec<T> / [T; n]比较，取决于长度和内容是否相等
    assert_eq!(&arr[..], vec);
    assert_eq!(&vec[..], arr);
}

fn print_slice<T: fmt::Debug>(s: &[T]) {
    println!("{:?}", s);
}

fn print_slice1<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("1: {:?}", s.as_ref());
}

#[test]
fn g() {
    let v = vec![1, 2, 3, 4];
    // Vec 实现了 Deref，&Vec 会被自动解引用为 &[T]，符合接口定义
    print_slice(&v);
    // 直接是 &[T]，符合接口定义
    print_slice(&v[..]);
    // &Vec 支持 AsRef<[T]>
    print_slice1(&v);
    // &[T] 支持 AsRef<[T]>
    print_slice1(&v[..]);
    //  Vec 支持 AsRef<[T]>
    print_slice1(v);

    // 数组虽没有实现 Deref，但它的解引用就是 &[T]
    let arr = [1, 2, 3, 4];
    print_slice(&arr);
    print_slice(&arr[..]);

    print_slice1(&arr);
    print_slice1(&arr[..]);
    print_slice1(arr);
}

#[test]
fn iter_std() {
    // 这里 Vec<T> 在调用 iter() 时被解引用成 &[T]，所以可以访问 iter()
    let result = vec![1, 2, 3, 4]
        .iter()
        .map(|v| v * v)
        .filter(|v| *v < 16)
        // 取前n个元素
        .take(1)
        .collect::<Vec<_>>();

    println!("{:?}", result);
}

use itertools::Itertools;
#[test]
fn iter_tools() {
    let err_str = "bad happened";
    let input = vec![Ok(21), Err(err_str), Ok(7)];
    let it = input
        .into_iter()
        .filter_map_ok(|i| if i > 10 { Some(i * 2) } else { None });
    // 结果应该是：vec![Ok(42), Err(err_str)]
    println!("{:?}", it.collect::<Vec<_>>());
}

#[test]
fn test_str() {
    let s = String::from("hello");
    // &String 会被解引用成 &str
    print_str_slice(&s);
    // &s[..] 和 s.as_str() 一样，都会得到 &str
    print_str_slice(&s[..]);

    // String 支持 AsRef<str>
    // print_str_slice1::<_, str>(&s);
    print_str_slice1::<_, [u8]>(&s[..]);
    // print_str_slice1(s.clone());

    // String 也实现了 AsRef<[u8]>，所以下面的代码成立
    // 打印出来是 [104, 101, 108, 108, 111]
    print_str_slice2(&s);
    print_str_slice2(&s[..]);
    print_str_slice2(s);
}

fn print_str_slice(s: &str) {
    println!("{:?}", s);
}

// fn print_str_slice1<T: AsRef<str>>(s: T) {
//     println!("1 {:?}", s.as_ref());
// }

fn print_str_slice1<T, U: ?Sized>(s: T)
where
    T: AsRef<U>,
    U: fmt::Debug,
{
    println!("1 {:?}", s.as_ref());
}

fn print_str_slice2<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("2 {:?}", s.as_ref());
}

use std::iter::FromIterator;
use std::ops::Deref;

#[test]
fn arr_slice_vs_str_slice() {
    let arr = ['h', 'e', 'l', 'l', 'o'];
    let vec = vec!['h', 'e', 'l', 'l', 'o'];
    let s = String::from("hello");
    let s1 = &arr[1..3];
    let s2 = &vec[1..3];
    // &str 本身就是一个特殊的 slice
    let s3 = &s[1..3];
    println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);
    // &[char] 和 &[char] 是否相等取决于长度和内容是否相等
    assert_eq!(s1, s2);
    // &[char] 和 &str 不能直接对比，我们把 s3 变成 Vec<char>
    assert_eq!(s2, s3.chars().collect::<Vec<_>>());
    // &[char] 可以通过迭代器转换成 String，String 和 &str 可以直接对比
    assert_eq!(String::from_iter(s2), s3);
}

#[test]
fn box_slice() {
    let mut v1 = vec![1, 2, 3, 4];
    v1.push(5);
    println!("cap should be 8: {}", v1.capacity());

    // 从 Vec<T> 转换成 Box<[T]>，此时会丢弃多余的 capacity
    let b1 = v1.into_boxed_slice();
    let mut b2 = b1.clone();

    let v2 = b1.into_vec();
    println!("cap should be exactly 5: {}", v2.capacity());

    assert!(b2.deref() == v2);

    // Box<[T]> 可以更改其内部数据，但无法 push
    b2[0] = 2;
    // b2.push(6);
    println!("b2: {:?}", b2);

    // 注意 Box<[T]> 和 Box<[T; n]> 并不相同
    let b3 = Box::new([2, 2, 3, 4, 5]);
    println!("b3: {:?}", b3);

    // b2 和 b3 相等，但 b3.deref() 和 v2 无法比较
    assert!(b2 == b3);
    // assert!(b3.deref() == v2);
}
