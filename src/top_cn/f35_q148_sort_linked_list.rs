use std::ops::RemAssign;

use crate::linked_list::ListNode;

// naive array sort
pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut arr = vec![];
    let mut ptr = &head;
    while let Some(node) = ptr {
        arr.push(node.val);
        ptr = &node.next;
    }
    arr.sort();
    let mut dummy = ListNode::new(-1);
    let mut ptr = &mut dummy.next;
    for num in arr {
        *ptr = Some(Box::new(ListNode::new(num)));
        ptr = &mut ptr.as_mut().unwrap().next;
    }
    dummy.next
}


pub fn sort_list_merge(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }
    let mut len = 0;
    let mut ptr = &head;
    while let Some(node) = ptr {
        len += 1;
        ptr = &node.next;
    }
    let mut ptr = &mut head;
    for _ in 0..len / 2 {
        if let Some(ref mut node) = ptr {
            ptr = &mut node.next;
        }
    }
    let mut next = ptr.take();
    let mut l1 = sort_list_merge(head);
    let mut l2 = sort_list_merge(next);
    merge(l1, l2)
}

fn merge(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
    match (l1, l2) {
        (None, Some(n)) | (Some(n), None) => Some(n),
        (Some(mut n1), Some(mut n2)) => {
            if n1.val < n2.val {
                n1.next = merge(n1.next.take(), Some(n2));
                Some(n1)
            } else {
                n2.next = merge(Some(n1), n2.next.take());
                Some(n2)
            }
        },
        (None, None) => None
    }
}
