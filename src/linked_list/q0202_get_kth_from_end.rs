use crate::linked_list::ListNode;



// type OptNode = Option<Box<ListNode>>;
pub fn kth_to_last(head: Option<Box<ListNode>>, k: i32) -> i32 {
    let mut slow = &head;
    let mut fast = &head;

    for _ in 0..k {
        if let Some(node) = fast {
            fast = &node.next;
        }
    }

    while fast.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next;
    }

    slow.as_ref().unwrap().val
}