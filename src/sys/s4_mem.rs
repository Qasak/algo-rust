use std::collections::HashMap;
use std::mem::{align_of, size_of};

struct S1 {
    a: u8,
    b: u16,
    c: u8,
}

struct S2 {
    a: u8,
    c: u8,
    b: u16,
}

#[repr(C)]
struct S3 {
    a: u8,
    b: u16,
    c: u8,
}

#[repr(C)]
struct S4 {
    a: u8,
    c: u8,
    b: u16,
}

enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}

// 这是一个声明宏，它会打印各种数据结构本身的大小，在 Option 中的大小，以及在 Result 中的大小
macro_rules! show_size {
    (header) => {
        println!(
            "{:<24} {:>4}    {}    {}",
            "Type", "T", "Option<T>", "Result<T, io::Error>"
        );
        println!("{}", "-".repeat(64));
    };
    ($t:ty) => {
        println!(
            "{:<24} {:4} {:8} {:12}",
            stringify!($t),
            size_of::<$t>(),
            size_of::<Option<$t>>(),
            size_of::<Result<$t, std::io::Error>>(),
        )
    };
}

#[cfg(test)]
mod test {
    use crate::sys::s4_mem::{E, S1, S2, S3, S4};
    use std::collections::HashMap;
    use std::mem::{align_of, size_of};

    #[test]
    fn g() {
        println!("sizeof S1: {}, S2: {}", size_of::<S1>(), size_of::<S2>());
        println!("alignof S1: {}, S2: {}", align_of::<S1>(), align_of::<S2>());
    }

    #[test]
    fn f() {
        println!("sizeof S3: {}, S4: {}", size_of::<S3>(), size_of::<S4>());
        println!("alignof S3: {}, S4: {}", align_of::<S3>(), align_of::<S4>());
    }

    #[test]
    fn show_test() {
        show_size!(header);
        show_size!(u8);
        show_size!(f64);
        show_size!(&u8);
        show_size!(Box<u8>);
        show_size!(&[u8]);

        show_size!(String);
        show_size!(Vec<u8>);
        show_size!(HashMap<String, String>);
        show_size!(E);
    }
}
