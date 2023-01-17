
struct SentenceIter<'a> {
    s: &'a mut &'a str,
    delimiter: char,
}

impl<'a> SentenceIter<'a> {
    pub fn new(s: &'a mut &'a str, delimiter: char) -> Self {
        Self { s, delimiter }
    }
}

impl<'a> Iterator for SentenceIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.s.find(self.delimiter) {
            None => None,
            Some(i) => {
                let s = &self.s[..i + self.delimiter.len_utf8()];
                *self.s = &self.s[i + self.delimiter.len_utf8()..];
                if let Some((start, _)) = s.as_bytes().iter().enumerate().find(|(_, &b)| b != b' ') {
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
    let mut s = "This is the 1st sentence. This is the 2nd sentence.";
    let mut iter = SentenceIter::new(&mut s, '.');
    assert_eq!(iter.next(), Some("This is the 1st sentence."));
    assert_eq!(iter.next(), Some("This is the 2nd sentence."));
    assert_eq!(iter.next(), None);
}

#[test]
fn g() {
    let mut s = "a。 b。 c";
    let sentences: Vec<_> = SentenceIter::new(&mut s, '。').collect();
    println!("sentences: {:?}", sentences);
}