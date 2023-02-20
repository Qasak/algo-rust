#[derive(Debug)]
struct SentenceIter<'b, 'a> {
    s: &'b mut &'a str,
    delimiter: char,
}

impl<'b, 'a> SentenceIter<'b, 'a> {
    pub fn new(s: &'b mut &'a str, delimiter: char) -> Self {
        Self { s, delimiter }
    }
}

impl<'b, 'a> Iterator for SentenceIter<'b, 'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.s.find(self.delimiter) {
            None => None,
            Some(i) => {
                let s = &self.s[..i + self.delimiter.len_utf8()];
                *self.s = &self.s[i + self.delimiter.len_utf8()..];
                if let Some((start, _)) = s.as_bytes().iter().enumerate().find(|(_, &b)| b != b' ')
                {
                    Some(&s[start..])
                } else {
                    None
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let s = "This is the 1st sentence. This is the 2nd sentence.".to_owned();
    let mut s1 = s.as_str();
    // 结构体的字段存在引用时，引用对应值的生命周期>=结构体的生命周期
    let mut iter = SentenceIter::new(&mut s1, '.');
    assert_eq!(iter.next(), Some("This is the 1st sentence."));
    assert_eq!(iter.next(), Some("This is the 2nd sentence."));
    println!("{}", s1);
    // assert_eq!(iter.next(), None);
}

#[test]
fn g() {
    let mut s = "a。 b。 c";
    let sentences: Vec<_> = SentenceIter::new(&mut s, '。').collect();
    println!("sentences: {:?}", sentences);
}
