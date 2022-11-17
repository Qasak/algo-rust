use std::borrow::Borrow;
use crate::top_cn::f2_q206_reverse_list::ListNode;



// type OptNode = Option<Box<ListNode>>;
pub fn kth_to_last(head: Option<Box<ListNode>>, k: i32) -> i32 {
    let mut slow = &head;
    let mut fast = &head;
    let x = fast.as_ref();
    let y = fast.unwrap();
    for _ in 0..k {
        if let Some(node) = fast {
            fast = &node.next;
            let x = &node.as_ref();
        }
    }
    while fast.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next;
    }
    slow.as_ref().unwrap().val
}