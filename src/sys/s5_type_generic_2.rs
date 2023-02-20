use std::io::{BufWriter, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

#[cfg(test)]
mod test {
    use crate::sys::s5_type_generic_2::MyWriter;
    use std::io::{BufWriter, Write};
    use std::net::TcpStream;

    #[test]
    fn ff() {
        let stream = TcpStream::connect("127.0.0.1:8088").unwrap();
        let writer = BufWriter::new(stream);
        let mut my_writer = MyWriter::new(writer);
        my_writer.write("ff!");
    }
}
