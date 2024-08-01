use crate::linked_list::*;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;
    while let Some(mut node) = curr {
        let next = node.next;
        node.next = prev;
        prev = Some(node);
        curr = next;
    }
    prev
}

pub fn reverse_list_rec(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    reverse(None, head)
}

pub fn reverse(pre: Option<Box<ListNode>>, cur: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(mut cur) = cur {
        let next = cur.next.take();
        cur.next = pre;
        reverse(Some(cur), next)
    } else {
        pre
    }
}

pub fn reverse_list_1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;
    while let Some(mut node) = curr {
        curr = node.next.replace(prev.unwrap());
        prev = Some(node);
    }
    prev
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn t() {
        let list1: MyOptBoxNode = vec![1, 2, 3, 4, 5].into();
        let list2: MyOptBoxNode = vec![1, 2].into();
        let list3: MyOptBoxNode = vec![].into();

        let l1 = list1.0;
        let l2 = list2.0;
        let l3 = list3.0;

        ListNode::print_list(&l1);
        ListNode::print_list(&l2);
        ListNode::print_list(&l3);

        let rl1 = reverse_list(l1);
        let rl2 = reverse_list(l2);
        let rl3 = reverse_list(l3);

        ListNode::print_list(&rl1);
        ListNode::print_list(&rl2);
        ListNode::print_list(&rl3);
    }
}
