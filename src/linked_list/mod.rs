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