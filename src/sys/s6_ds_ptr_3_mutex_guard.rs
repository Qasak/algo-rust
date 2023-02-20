use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::os::unix::raw::mode_t;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// lazy_static 宏可以生成复杂的 static 对象
// 其生命周期是静态的，不需要 Arc
// lazy_static!{
//     static ref METRICS: Mutex<HashMap<Cow<'static, str>, usize>> =
//         Mutex::new(HashMap::new());
// }

// 用once_cell::sync::Lazy取代lazy_static
static METRICS: Lazy<Mutex<HashMap<Cow<'static, str>, usize>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[test]
fn f() {
    // 用 Arc 来提供并发环境下的共享所有权（使用引用计数
    // let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    for _ in 0..32 {
        // let m = metrics.clone();
        let m = &METRICS;
        thread::spawn(move || {
            let mut g = m.lock().unwrap();
            let data = &mut *g;
            // let data = &mut g.deref_mut();

            // Cow 实现了很多数据结构的 From trait，
            // 所以我们可以用 "hello".into() 生成 Cow
            let entry = data.entry("hello".into()).or_insert(0);
            *entry += 1;
            // MutexGuard 被 Drop，锁被释放
        });
    }
    thread::sleep(Duration::from_millis(100));
    println!("metrics: {:?}", METRICS.lock().unwrap());
    // println!("metrics: {:?}", metrics.lock().unwrap());
}
