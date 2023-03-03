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