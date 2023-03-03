use crate::linked_list::ListNode;

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(node), None) | (None, Some(node)) => Some(node),
        (Some(mut node1), Some(mut node2)) => {
            if node1.val < node2.val {
                node1.next = merge_two_lists(node1.next, Some(node2));
                Some(node1)
            } else {
                node2.next = merge_two_lists(Some(node1), node2.next);
                Some(node2)
            }
        }
    }
}

pub fn merge_two_lists_iter(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0);
    let mut tail = &mut dummy_head;
    let (mut p1, mut p2) = (l1, l2);

    while p1.is_some() && p2.is_some() {
        let mut n1 = p1.unwrap();
        let mut n2 = p2.unwrap();
        if n1.val < n2.val {
            let next = n1.next.take();
            tail.next = Some(n1);
            p1 = next;
            p2 = Some(n2);
        } else {
            let next = n2.next.take();
            tail.next = Some(n2);
            p2 = next;
            p1 = Some(n1);
        }
        tail = tail.next.as_mut().unwrap();

    }

    if p1.is_some() {
        tail.next = p1;
    }
    if p2.is_some() {
        tail.next = p2;
    }

    dummy_head.next
}
#[test]
fn test_merge() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let merged = merge_two_lists_iter(l1, l2);
    ListNode::print_list(&merged);
}
