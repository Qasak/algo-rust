use std::ops::Deref;

mod q876_middle_of_linked_list;
mod q0202_get_kth_from_end;
mod d_list;
mod q141_has_cycle;
mod q25_k_reverse;
mod q206_reverse_linked_list;
mod q1669_merge_linked_list;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub(crate) fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl ListNode {
    pub fn add_node(&mut self, val: i32) {
        let mut current_node = self;
        while let Some(ref mut next_node) = current_node.next {
            // 这里应该是用到了Deref Coercion自动解引用
            // current_node = &mut **next_node;
            current_node = next_node;

        }
        current_node.next = Some(Box::new(ListNode::new(val)));
    }
}
