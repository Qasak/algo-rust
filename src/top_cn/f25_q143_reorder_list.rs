use std::collections::VecDeque;
use crate::linked_list::ListNode;

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut dq = VecDeque::new();
    let mut tail_ptr = &mut head.as_mut().unwrap().next;
    let mut tail_node = tail_ptr.take();
    while let Some(mut node) = tail_node {
        let next = node.next.take();
        dq.push_back(Some(node));
        tail_node = next;
    }
    let mut flag = true;
    while !dq.is_empty() {
        *tail_ptr = if flag {
            dq.pop_back().unwrap()
        } else {
            dq.pop_front().unwrap()
        };
        tail_ptr = &mut tail_ptr.as_mut().unwrap().next;
        flag = !flag;
    }
}

#[test]
fn reorder_list_test() {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut ptr = &mut head;
    for i in 2..=5 {
        ptr.as_mut().unwrap().next = Some(Box::new(ListNode::new(i)));
        ptr = &mut ptr.as_mut().unwrap().next;
    }
    reorder_list(&mut head);
    ListNode::print_list(&head);
    eprintln!("head = {:?}", head);
}
