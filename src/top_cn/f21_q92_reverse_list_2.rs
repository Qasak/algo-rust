use crate::linked_list::ListNode;

pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    dummy.next = head;
    let mut ptr = &mut dummy;

    for _ in 1..left {
        ptr = ptr.next.as_mut().unwrap();
    }

    let mut cur = ptr.next.take();
    let mut pre = None;
    for _ in left..=right {
        if let Some(mut node) = cur {
            let next = node.next.take();
            node.next = pre;
            pre = Some(node);
            cur = next;
        }
    }
    ptr.next = pre;
    for _ in left..=right {
        ptr = ptr.next.as_mut().unwrap();
    }

    ptr.next = cur;
    dummy.next
}
