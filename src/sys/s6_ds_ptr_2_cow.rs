use std::borrow::Cow;
use url::Url;

#[test]
fn f() {
    let url = Url::parse("https://github.com/qasak?page=0&sort=desc&extra=nihao%20sijai").unwrap();
    let mut pairs = url.query_pairs();

    assert_eq!(pairs.count(), 3);

    let (mut k, v) = pairs.next().unwrap();
    // 因为 k, v 都是 Cow<str> 他们用起来感觉和 &str 或者 String 一样
    // 此刻，他们都是 Borrowed
    print_pairs((k.clone(), v.clone()));
    // 当修改发生时，k 变成 Owned
    k.to_mut().push_str("_lala");

    print_pairs((k, v));

    print_pairs(pairs.next().unwrap());
    // 在处理 extra=hello%20world 时，value 被处理成 "hello world"
    // 所以这里 value 是 Owned
    print_pairs(pairs.next().unwrap());
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Borrowed {}", v),
        Cow::Owned(v) => format!("Owned {}", v),
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
struct User<'input> {
    // 必须加上标记，否则是owned
    #[serde(borrow)]
    name: Cow<'input, str>,
    age: u8,
}
#[test]
fn g() {
    let input = r#"{ "name": "Mario", "age": 18 }"#;
    let user: User = serde_json::from_str(input).unwrap();

    match user.name {
        Cow::Borrowed(x) => println!("borrowed {}", x),
        Cow::Owned(x) => println!("owned {}", x),
    }
}
