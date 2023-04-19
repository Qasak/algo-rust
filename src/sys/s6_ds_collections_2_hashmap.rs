use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[test]
fn f() {
    let map = HashMap::new();
    let mut map = explain("empty", map);

    map.insert('a', 1);
    let mut map = explain("added 1", map);
    map.insert('b', 2);
    map.insert('c', 3);

    let mut map = explain("added 3", map);

    map.insert('d', 4);

    let mut map = explain("added 4", map);

    map.remove(&'a');

    explain("final", map);
}

fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
    let arr: [usize; 6] = unsafe { std::mem::transmute(map) };
    println!(
        "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: {}, items: {}",
        name, arr[2], arr[3], arr[4], arr[5]
    );
    unsafe { std::mem::transmute(arr) }
}

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Name {
    pub name: String,
    pub flags: u32,
}

impl Name {
    pub fn new(name: impl AsRef<str>, flags: u32) -> Self {
        Self {
            name: name.as_ref().to_string(),
            flags,
        }
    }
}

#[test]
fn g() {
    let mut map = BTreeMap::new();
    map.insert(Name::new("/tmp/password", 0x1), 1);
    map.insert(Name::new("/etc/hosts", 0x2), 12);
    map.insert(Name::new("/etc/hosts", 0x1), 4);
    map.insert(Name::new("/etc/host", 0x1), 4);
    map.insert(Name::new("/home/tchen", 0x0), 28);

    for item in map.iter() {
        println!("BTreeMap: {:?}", item);
    }

    let mut map1 = HashMap::new();
    map1.insert(Name::new("/tmp/password", 0x1), 1);
    map1.insert(Name::new("/etc/hosts", 0x2), 12);
    map1.insert(Name::new("/etc/hosts", 0x1), 4);
    map1.insert(Name::new("/etc/host", 0x1), 4);
    map1.insert(Name::new("/home/tchen", 0x0), 28);
    for item in map1.iter() {
        println!("HashMap: {:?}", item);
    }
}
