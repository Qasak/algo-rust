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

pub fn merge_two_lists_iter(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(-1);
    let mut cur = &mut dummy;
    while let (Some(n1), Some(n2)) = (list1.as_ref(), list2.as_ref()) {
        let (v1, v2) = (n1.val, n2.val);
        let l = if v1 < v2 { &mut list1 } else { &mut list2 };
        cur.next = l.take();
        cur = cur.next.as_mut().unwrap();
        *l = cur.next.take();
    }
    cur.next = list1.or(list2);
    dummy.next
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
