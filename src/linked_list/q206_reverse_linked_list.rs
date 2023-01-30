use crate::linked_list::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut pre = None;
    while let Some(mut cur_node) = cur {
        let nex = cur_node.next;
        cur_node.next = pre;
        pre = Some(cur_node);
        cur = nex;
    }
    pre
}