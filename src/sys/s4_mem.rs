
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


#[cfg(test)]
mod test {
    use std::mem::{align_of, size_of};
    use crate::sys::s4_mem::{S1, S2, S3, S4};

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
}