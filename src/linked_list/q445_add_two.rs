use std::ops::{AddAssign, Deref};

type NodePtr<T> = Option<Box<Node<T>>>;
type NodeRef<'a, T> = Option<&'a Node<T>>;
struct Node<T> {
    data: T,
    next: NodePtr<T>,
}

impl<T> Node<T> {
    fn new(data: T, next: NodePtr<T>) -> Self {
        Self { data, next }
    }
    fn from_vec(vec: Vec<T>) -> NodePtr<T> {
        let mut head = None;
        for elem in vec.into_iter().rev() {
            let node = Box::new(Node::new(elem, head.take()));
            head.replace(node);
        }
        head
    }
}

fn into_vec<T>(mut head: NodePtr<T>) -> Vec<T> {
    let mut result = Vec::new();
    while let Some(node) = head.take() {
        result.push(node.data);
        head = node.next;
    }
    result
}

fn len<T>(head: NodeRef<'_, T>) -> usize {
    head.map(|head| len(head.next.as_deref()) + 1)
        .unwrap_or_default()
}

fn add<T: Copy + AddAssign<T>>(a: NodePtr<T>, b: NodePtr<T>) -> NodePtr<T> {
    match (a, b) {
        (Some(a), Some(b)) => {
            let a_len = len(Some(a.deref()));
            let b_len = len(Some(b.deref()));
            let (a, b, a_len, b_len) = if a_len > b_len {
                (a, b, a_len, b_len)
            } else {
                (b, a, b_len, a_len)
            };
            let mut head = Some(a);
            let mut head_ref = head.as_mut().unwrap().as_mut();
            for _ in 0..(a_len - b_len) {
                head_ref = head_ref.next.as_mut().unwrap().as_mut();
            }
            let mut b_ref = b.as_ref();
            for i in 0..b_len {
                head_ref.data += b_ref.data;
                if i + 1 < b_len {
                    head_ref = head_ref.next.as_mut().unwrap().as_mut();
                    b_ref = b_ref.next.as_ref().unwrap().as_ref();
                }
            }
            head
        }
        (Some(a), None) | (None, Some(a)) => Some(a),
        (None, None) => None,
    }
}

#[test]
fn test_list() {
    let a = Node::from_vec(vec![1, 2, 3, 4]);
    let b = Node::from_vec(vec![2, 3, 4]);
    assert_eq!(into_vec(add(a, b)), vec![1, 4, 6, 8]);
}
