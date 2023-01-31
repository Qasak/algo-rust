use std::{collections::HashMap, mem::size_of_val, mem::size_of};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[test]
fn f() {
    // 长度0
    let c1 = || println!("nihao!");
    // 长度0 参数无关
    let c2 = |i: i32| println!("nihao {}", i);
    let name = "jas".to_owned();
    let name1 = name.clone();
    let mut map = HashMap::new();
    map.insert("ni", "hao");
    // 捕获一个引用，长度8
    let c3 = || println!("hello:{}", name);
    // 捕获移动的数据， name1(24) + map(48) closure长度72
    let c4 = move || println!("hello: {}, {:?}", name1, map);
    let name2 = name.clone();
    // 和局部变量无关，捕获一个String，长度24
    let c5 = move || {
        let x = 1;
        let name3 = "ling".to_owned();
        println!("hello: {}, {:?}, {:?}", x, name2, name3);

    };
    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, f: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&f),
    );

    println!("map {}", size_of::<HashMap<&str, &str>>());
    println!("u64 {}", size_of::<u64>());
    println!("NonNull<u8> {}", size_of::<NonNull<u8>>());
    println!("usize {}", size_of::<usize>());
    println!("u32 {}", size_of::<u32>());
    println!("i32 {}", size_of::<i32>());
    println!("PhantomData<T> {}", size_of::<PhantomData<i32>>());

}

#[test]
fn g() {
    let name = String::from("Tyr");

    // 这个闭包会 clone 内部的数据返回，所以它不是 FnOnce
    let c = move |greeting: String| (greeting, name.clone());

    // 所以 c1 可以被调用多次

    println!("c1 call once: {:?}", c("qiao".into()));
    println!("c1 call twice: {:?}", c("bonjour".into()));

    // 然而一旦它被当成 FnOnce 被调用，就无法被再次调用
    println!("result: {:?}", call_once("hi".into(), c));

    // 无法再次调用
    // let result = c("hi".to_string());

    // Fn 也可以被当成 FnOnce 调用，只要接口一致就可以
    println!("result: {:?}", call_once("hola".into(), not_closure));
}

fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(arg)
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "Rosie".into())
}