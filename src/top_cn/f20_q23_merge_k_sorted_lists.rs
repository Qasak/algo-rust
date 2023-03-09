use std::{cmp::Ordering, collections::BinaryHeap};

use crate::linked_list::ListNode;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val).reverse()
    }
}
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap = BinaryHeap::new();
    for head in lists {
        if head.is_some() {
            heap.push(head);
        }
    }
    let mut dummy = ListNode::new(-1);
    let mut p = &mut dummy;
    while let Some(mut head) = heap.pop() {
        let next = head.as_mut().unwrap().next.take();
        p.next = head;
        p = p.next.as_mut().unwrap();
        if next.is_some() {
            heap.push(next);
        }
    }
    dummy.next
}

pub fn merge_k_lists_rec(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.len() == 0 {
        return None;
    }

    merge(&mut lists)
}
fn merge(lists: &mut [Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
    if lists.len() == 1 {
        if lists[0].is_some() {
            return lists[0].take();
        } else {
            return None;
        }
    }
    let m = lists.len() / 2;
    let l1 = merge(&mut lists[..m]);
    let l2 = merge(&mut lists[m..]);
    merge_2_lists(l1, l2)
}
fn merge_2_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(mut n1), Some(mut n2)) => {
            if n1.val < n2.val {
                n1.next = merge_2_lists(n1.next.take(), Some(n2));
                Some(n1)
            } else {
                n2.next = merge_2_lists(Some(n1), n2.next.take());
                Some(n2)
            }
        }
        _ => None,
    }
}

mod test {
    use super::*;
    fn cmp_nodes(node1: &Option<Box<ListNode>>, node2: &Option<Box<ListNode>>) -> Ordering {
        // 如果其中一个节点为空，则另一个节点更小
        if node1.is_none() {
            return Ordering::Less;
        } else if node2.is_none() {
            return Ordering::Greater;
        }

        // 获取两个节点的值，并比较它们
        let val1 = node1.as_ref().unwrap().val;
        let val2 = node2.as_ref().unwrap().val;
        val1.cmp(&val2).reverse() // 由于是最小堆，因此需要将比较结果反转
    }
    #[test]
    fn bh_work() {
        let mut heap = BinaryHeap::new();

        let node1 = Some(Box::new(ListNode { val: 1, next: None }));
        let node2 = Some(Box::new(ListNode { val: 2, next: None }));
        let node3 = Some(Box::new(ListNode { val: 3, next: None }));

        heap.push(&node2);
        heap.push(&node1);
        heap.push(&node3);

        while let Some(node) = heap.pop() {
            println!("{:?}", node);
        }
    }
}
