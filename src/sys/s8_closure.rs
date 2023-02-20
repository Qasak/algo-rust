use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{collections::HashMap, mem::size_of, mem::size_of_val};

#[test]
fn test_fn_once() {
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
        size_of_val(&test_fn_once),
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
fn test_call_once() {
    let name = String::from("Tyr");

    // 这个闭包会 clone 内部的数据返回，所以它不是 FnOnce
    // 所以 c1 可以被调用多次
    // 但是name仍然被强制move了，如果后续再使用会报错
    let c = move |greeting: String| (greeting, name.clone());
    // println!("{}", name);

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

#[test]
fn test_fn_mut() {
    let mut name = String::from("hello");
    let mut name1 = String::from("hola");

    // 捕获 &mut name
    let mut c = || {
        name.push_str(" Tyr");
        println!("c: {}", name);
    };

    // 捕获 mut name1，注意 name1 需要声明成 mut
    let mut c1 = move || {
        name1.push_str("!");
        println!("c1: {}", name1);
    };

    c();
    c1();

    call_mut(&mut c);
    call_mut(&mut c1);

    call_once_1(c);
    call_once_1(c1);
}

// 在作为参数时，FnMut 也要显式地使用 mut，或者 &mut
fn call_mut(c: &mut impl FnMut()) {
    c();
}

// 由于FnOnce trait中的call_once函数签名的第一个参数是 self，它会转移 self 的所有权到 call_once 函数中。
// extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
// 所以这里给多态参数时不需要 &mut, 如果给的是 &mut c, 则会 mismatched types

// 至于
// call_once_1(mut c: impl FnOnce())
// call_once_1(c: impl FnOnce())
// 两种都是可以的
fn call_once_1(c: impl FnOnce()) {
    c();
}

#[test]
fn test_fn() {
    let v = vec![0u8; 1024];
    let v1 = vec![0u8; 1023];
    // Fn，不移动所有权
    let mut c = |x: u64| v.len() as u64 * x;
    // Fn，移动所有权
    let mut c1 = move |x: u64| v1.len() as u64 * x;

    println!("direct call: {}", c(2));
    println!("direct call: {}", c1(2));

    println!("call: {}", call(3, &c));
    println!("call: {}", call(3, &c1));

    println!("call_mut: {}", call_mut_2(4, &mut c));
    println!("call_mut: {}", call_mut_2(4, &mut c1));

    println!("call_once: {}", call_once_2(5, c));
    println!("call_once: {}", call_once_2(5, c1));
}

fn call(arg: u64, c: &impl Fn(u64) -> u64) -> u64 {
    c(arg)
}

fn call_mut_2(arg: u64, c: &mut impl FnMut(u64) -> u64) -> u64 {
    c(arg)
}

fn call_once_2(arg: u64, c: impl FnOnce(u64) -> u64) -> u64 {
    c(arg)
}

use std::ops::Mul;
#[test]
fn test_closure_ret() {
    let c1 = curry(5);
    println!("5 multiply 2 is: {}", c1(2));

    let adder2 = curry(3.14);
    println!("pi multiply 4^2 is: {}", adder2(4. * 4.));
}

fn curry<T>(x: T) -> impl Fn(T) -> T
where
    T: Mul<Output = T> + Copy,
{
    move |y| x * y
}

// struct Closure<'a, 'b: 'a> {
// 	data: (i32, i32, i32, i32),
// 	v: &'a [&'b str],
// 	name: String,
// }
#[test]
fn test_closure_lifetime() {
    let name = String::from("Tyr");
    let vec = vec!["Rust", "Elixir", "Javascript"];
    let v = &vec[..];
    let data = (1, 2, 3, 4);
    let c = move || {
        println!("data: {:?}", data);
        println!("v: {:?}, name: {:?}", v, name.clone());
    };
    c();
    // 请问在这里，还能访问 name 么？为什么？
}

pub trait Executor {
    fn execute(&self, cmd: &str) -> Result<String, &'static str>;
}

struct BashExecutor {
    env: String,
}

impl Executor for BashExecutor {
    fn execute(&self, cmd: &str) -> Result<String, &'static str> {
        Ok(format!(
            "fake bash execute: env: {}, cmd: {}",
            self.env, cmd
        ))
    }
}

impl<F> Executor for F
where
    F: Fn(&str) -> Result<String, &'static str>,
{
    fn execute(&self, cmd: &str) -> Result<String, &'static str> {
        self(cmd)
    }
}

// line let r2 = execute(...);
#[test]
fn closure_trait() {
    let env = "PATH=/usr/bin".to_string();

    let cmd = "cat /etc/passwd";
    let r1 = execute(cmd, BashExecutor { env: env.clone() });
    println!("{:?}", r1);

    let r2 = execute(cmd, |cmd: &str| {
        Ok(format!("fake fish execute: env: {}, cmd: {}", env, cmd))
    });
    println!("{:?}", r2);
}

fn execute(cmd: &str, exec: impl Executor) -> Result<String, &'static str> {
    exec.execute(cmd)
}

// 闭包模拟
struct ClosureOnce<Captured, Args, Output> {
    // 捕获的数据
    captured: Captured,
    // closure 的执行代码
    func: fn(Args, Captured) -> Output,
}

impl<Captured, Args, Output> ClosureOnce<Captured, Args, Output> {
    // 模拟 FnOnce 的 call_once，直接消耗 self
    fn call_once(self, greeting: Args) -> Output {
        (self.func)(greeting, self.captured)
    }
}

// 类似 greeting 闭包的函数体
fn greeting_code1(args: (String,), captured: (String,)) -> (String, String) {
    (args.0, captured.0)
}

fn greeting_code2(args: (String, String), captured: (String, u8)) -> (String, String, String, u8) {
    (args.0, args.1, captured.0, captured.1)
}

#[test]
fn closure_simulate() {
    let name = "Tyr".into();
    // 模拟变量捕捉
    let c = ClosureOnce {
        captured: (name,),
        func: greeting_code1,
    };

    // 模拟闭包调用，这里和 FnOnce 不完全一样，传入的是一个 tuple 来匹配 Args 参数
    println!("{:?}", c.call_once(("hola".into(),)));
    // 调用一次后无法继续调用
    // println!("{:?}", clo.call_once("hola".into()));

    // 更复杂一些的复杂的闭包
    let c1 = ClosureOnce {
        captured: ("Tyr".into(), 18),
        func: greeting_code2,
    };

    println!("{:?}", c1.call_once(("hola".into(), "hallo".into())));
}
