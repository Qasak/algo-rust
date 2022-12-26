use crate::top_cn::f2_q206_reverse_list::ListNode;

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut remain = head;
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    while remain.is_some() {
        let (new_head, new_remain) = reverse_one(remain, k);
        remain = new_remain;
        tail.next = new_head;
        while tail.next.as_ref().is_some() {
            tail = tail.next.as_mut().unwrap();
        }
    }

    dummy.next
}

// 反转一次，返回反转后的head和remain
// 如果为最后一次不足以反转，remain为None
fn reverse_one(head: Option<Box<ListNode>>, k: i32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut cur = head.as_ref();
    for _ in 0..k {
        if cur.is_none() {
            return (head, None);
        }
        cur = cur.unwrap().next.as_ref();
    }

    let mut cur = head;
    let mut pre = None;
    for _ in 0..k {
        if let Some(mut node) = cur {
            let nex = node.next;
            node.next = pre;
            pre = Some(node);
            cur = nex;
        }
    }
    (pre, cur)
}