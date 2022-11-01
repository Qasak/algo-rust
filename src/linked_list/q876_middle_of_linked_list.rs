use crate::top_cn::f2_q206_reverse_list::ListNode;

pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();
    loop {
        if let Some(node) = fast {
            fast = node.next.as_ref();
        } else {
            break;
        }
        if let Some(node) = fast {
            fast = node.next.as_ref();
        } else {
            break;
        }
        if let Some(node) = slow {
            slow = node.next.as_ref();
        } else {
            break;
        }
    }
    // equal to `let mid_addr = slow.unwrap().as_ref() as *const ListNode;`
    let mid_addr = if let Some(node) = slow {
        // node.as_ref() is a &ListNode
        node.as_ref() as * const ListNode
    } else {
        return None;
    };

    while let Some(node) = head.as_ref() {
        let addr = node.as_ref() as *const ListNode;
        if addr != mid_addr {
            head = head.unwrap().next;
        } else {
            break;
        }
    }
    head
}

pub fn middle_node_1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut slow, mut fast) = (&head, &head);
    // fast.as_ref() is Option<&Box<ListNode>>
    // `fast.as_ref()?.next` equal to `fast.as_ref().unwrap().next`, it's an Option<Box<ListNode>>
    while fast.as_ref().is_some() && fast.as_ref()?.next.is_some() {
        slow = &slow.as_ref()?.next;
        fast = &fast.as_ref()?.next.as_ref()?.next;
    }
    // .clone() makes &Option<Box<ListNode>> to Option<Box<ListNode>>
    slow.clone()
}
