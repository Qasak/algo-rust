
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


#[cfg(test)]
mod test {
    use std::mem::{align_of, size_of};
    use crate::sys::s4_mem::{S1, S2};

    #[test]
    fn g() {
        println!("sizeof S1: {}, S2: {}", size_of::<S1>(), size_of::<S2>());
        println!("alignof S1: {}, S2: {}", align_of::<S1>(), align_of::<S2>());
    }
}