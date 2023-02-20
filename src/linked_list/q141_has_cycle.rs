// code and test form: https://leetcode.com/problems/linked-list-cycle/solutions/2071179/rust-solution/?languageTags=rust

use std::{cell::RefCell, collections::HashSet, rc::Rc};
// Custom ListNode with Rc & RefCell
// to allow for cycles
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;
impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
        let mut cache = HashSet::new();
        let mut next = head;

        while let Some(node) = next {
            let mut n = node.borrow_mut();
            if !cache.insert(n.val) {
                return true;
            }

            next = n.next.take();
        }
        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_1() {
        let a = Rc::new(RefCell::new(ListNode::new(3)));
        let b = Rc::new(RefCell::new(ListNode::new(2)));
        let c = Rc::new(RefCell::new(ListNode::new(0)));
        let d = Rc::new(RefCell::new(ListNode::new(4)));
        a.borrow_mut().next = Some(b.clone());
        b.borrow_mut().next = Some(c.clone());
        c.borrow_mut().next = Some(d.clone());
        d.borrow_mut().next = Some(a.clone());
        assert_eq!(Solution::has_cycle(Some(a)), true);
    }

    #[test]
    fn test_2() {
        let a = Rc::new(RefCell::new(ListNode::new(1)));
        let b = Rc::new(RefCell::new(ListNode::new(2)));
        a.borrow_mut().next = Some(b.clone());
        b.borrow_mut().next = Some(a.clone());
        assert_eq!(Solution::has_cycle(Some(a)), true);
    }

    #[test]
    fn test_3() {
        let a = Rc::new(RefCell::new(ListNode::new(1)));
        assert_eq!(Solution::has_cycle(Some(a)), false);
    }
}
