//  (signal: 4, SIGILL: illegal instruction)
//  Process finished with exit code 101

// use std::alloc::{GlobalAlloc, Layout, System};
//
// struct MyAllocator;
//
// unsafe impl GlobalAlloc for MyAllocator {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         let data = System.alloc(layout);
//         format!("ALLOC: {:p}, size {}", data, layout.size());
//         data
//     }
//
//     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//         System.dealloc(ptr, layout);
//         format!("FREE: {:p}, size {}", ptr, layout.size());
//     }
// }
//
// #[global_allocator]
// static GLOBAL: MyAllocator = MyAllocator;
//
// #[allow(dead_code)]
// struct Matrix {
//     // 使用不规则的数字如 505 可以让 dbg! 的打印很容易分辨出来
//     data: [u8; 505],
// }
//
// impl Default for Matrix {
//     fn default() -> Self {
//         Self { data: [0; 505] }
//     }
// }

#[test]
fn my_allocator_test() {
    // let data = Box::new(Matrix::default());
    // println!(
    //     "!!! allocated memory: {:p}, len: {}",
    //     &*data,
    //     std::mem::size_of::<Matrix>()
    // )
    format!("{}", 1);
}

fn box_test() {
    // 在堆上分配 16M 内存，但它会现在栈上出现，再移动到堆上
    let boxed = Box::new([0u8; 1 << 24]);
    println!("len: {}", boxed.len());
}
