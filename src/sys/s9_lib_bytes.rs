use bytes::{Buf, BufMut, Bytes, BytesMut};
#[test]
fn f() {

    let mut buf = BytesMut::with_capacity(1024);
    buf.put(b"hello world".as_slice());
    buf.put_u16(1234);

    let a = buf.split();
    assert_eq!(a, b"hello world\x04\xD2"[..]);

    buf.put(&b"goodbye world"[..]);

    let b = buf.split();
    assert_eq!(b, b"goodbye world"[..]);

    assert_eq!(buf.capacity(), 998);

}



#[test]
fn test_u8_slice_buf_trait() {

    use bytes::Buf;

    let mut buf = b"hello world".as_slice();

    assert_eq!(b'h', buf.get_u8());
    assert_eq!(b'e', buf.get_u8());
    assert_eq!(b'l', buf.get_u8());

    let mut rest = [0; 8];
    buf.copy_to_slice(&mut rest);

    assert_eq!(&rest[..], &b"lo world"[..]);
}

#[test]
fn test_buf_struct() {

    let mut mem = Bytes::from("Hello world");
    let a = mem.slice(0..5);

    assert_eq!(a, "Hello");
    let b = mem.split_to(6);

    assert_eq!(mem, "world");
    assert_eq!(b, "Hello ");
}