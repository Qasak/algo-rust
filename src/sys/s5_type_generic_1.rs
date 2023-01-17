
use std::fs::File;
use std::io::{BufReader, Read, Result};

// 定义一个带有泛型参数 R 的 reader，此处我们不限制 R
#[derive(Debug)]
struct MyReader<R> {
    reader: R,
    buf: String,
}

// 实现 new 函数时，我们不需要限制 R
impl<R> MyReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::with_capacity(1024),
        }
    }
}

// 定义 process 时，我们需要用到 R 的方法，此时我们限制 R 必须实现 Read trait
// impl<R: Read> MyReader<R>
//     // where
//     //     R: Read,
// {
//     pub fn process(&mut self) -> Result<usize> {
//         self.reader.read_to_string(&mut self.buf)
//     }
// }

impl<R> MyReader<R>
where
    R: Read,
{
    pub fn process(&mut self) -> Result<usize> {
        self.reader.read_to_string(&mut self.buf)
    }
}
#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;
    use crate::sys::s5_type_generic_1::MyReader;

    #[test]
    fn read_it() {
        let f = File::open("/etc/hosts").unwrap();
        let mut reader = MyReader::new(BufReader::new(f));
        let size = reader.process().unwrap();
        println!("total size read: {}", size);
        println!("{reader:?}");
    }
}
