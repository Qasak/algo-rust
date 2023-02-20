// use std::collections::HashMap;
// struct LRUCache {
//     k_node_map: HashMap<i32, Option<Box<Node>>>,
//     size: usize,
//     capacity: usize,
//     head: Option<Box<Node>>,
//     tail: Option<Box<Node>>,
// }
//
// #[derive(PartialEq, Eq, Clone, Debug)]
// struct Node {
//     k: i32,
//     v: i32,
//     next: Option<Box<Node>>,
//     prev: Option<Box<Node>>
// }
// impl Node {
//     #[inline]
//     fn new(k: i32, v: i32) -> Self {
//         Node {k, v ,next: None, prev: None}
//     }
// }
//
// /**
//   * `&self` means the method takes an immutable reference
//   * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl LRUCache {
//
//     fn new(capacity: i32) -> Self {
//         let mut head = Box::new(Node::new(-1, -1));
//         let mut tail = Box::new(Node::new(-1, -1));
//         // clone ptr
//         head.next = Some(tail.clone());
//         tail.prev = Some(head.clone());
//         LRUCache {
//             k_node_map: HashMap::new(),
//             size: 0,
//             capacity: capacity as usize,
//             head : Some(head),
//             tail : Some(tail),
//         }
//     }
//
//     fn get(&mut self, key: i32) -> i32 {
//
//     }
//
//     fn put(&mut self, key: i32, value: i32) {
//
//     }
//
//     fn move_to_head(&self, node: Option<Box<Node>>) {
//
//     }
//
//     fn remove_tail(&mut self) {
//
//     }
// }
