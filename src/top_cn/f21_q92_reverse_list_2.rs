use crate::linked_list::ListNode;

pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    dummy.next = head;
    let mut ptr = &mut dummy;

    for _ in 1..left {
        ptr = ptr.next.as_mut().unwrap();
    }



    if let Some(mut pre_node) = ptr.next.take() {
        let mut cur = pre_node.next.take();

        for _ in left..right {
            let mut cur_node = cur.unwrap();
            cur = cur_node.next.replace(pre_node);
            pre_node = cur_node;
        }

        // for _ in left..right {
        //     let mut cur_node = cur.unwrap();
        //     let next = cur_node.next.take();
        //     cur_node.next = Some(pre_node);
        //     pre_node = cur_node;
        //     cur = next;
        // }

        ptr.next = Some(pre_node);

        for _ in left..=right {
            ptr = ptr.next.as_mut().unwrap();
        }

        ptr.next = cur;
    }
    dummy.next
}