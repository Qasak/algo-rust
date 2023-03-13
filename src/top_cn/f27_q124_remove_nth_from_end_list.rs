use crate::linked_list::{ListNode, MyOptBoxNode};

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut fast = &mut head;
    for _ in 0..=n {
        // 如果去掉else
        // cannot borrow `*fast` as mutable more than once at a time (solution.rs)
        if let Some(node) = fast.as_mut() {
            fast = &mut node.next;
        } else {
            return head.unwrap().next;
        }
    }
    let mut cnt = 0;
    while let Some(mut fast_node) = fast.as_mut() {
        fast = &mut fast_node.next;
        cnt += 1;
    };
    // NLL规则
    let mut slow = &mut head;
    for _ in 0..cnt {
        slow = &mut slow.as_mut().unwrap().next;
    }
    let next = slow.as_mut().unwrap().next.take();
    slow.as_mut().unwrap().next = next.unwrap().next;
    head
}


pub fn remove_nth_from_end_1(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode{val: -1, next: head};
    let mut fast = &mut dummy;
    for _ in 0..n {
        if let Some(ref mut fast_node) = fast.next {
            fast = fast_node;
        }
    }
    let mut cnt = 0;
    while let Some(ref mut fast_node) = fast.next {
        fast = fast_node;
        cnt += 1;
    }
    let mut slow = &mut dummy;
    for _ in 0..cnt {
        slow = slow.next.as_mut().unwrap();
    }
    let mut next = slow.next.as_mut().take();
    slow.next = if let Some(ref mut next_node) = next {
        next_node.next.take()
    } else {
        None
    };
    dummy.next
}
#[test]
fn remove_nth_from_end_work() {
    // test remove_nth_from_end
    let mut list: MyOptBoxNode = vec![1,2,3,4,5].into();
    let mut head = list.0;
    let ret = remove_nth_from_end(head, 5);
    ListNode::print_list(&ret);
}