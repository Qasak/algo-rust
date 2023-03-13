use std::{rc::Rc, cell::RefCell, collections::VecDeque};

use crate::tree::TreeNode;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = vec![];
    if root.is_none() {
        return ret;
    }
    let mut cur = root.unwrap();
    let mut dq = VecDeque::new();
    dq.push_back(cur);
    while !dq.is_empty() {
        let n = dq.len();
        for i in 0..n {
            cur = dq.pop_front().unwrap();
            if i == n - 1 {
                ret.push(cur.borrow().val);
            }
            if let Some(left_node) = cur.borrow_mut().left.take() {
                dq.push_back(left_node);
            }

            if let Some(right_node) = cur.borrow_mut().right.take() {
                dq.push_back(right_node);
            }
        }
        
    }
    ret
}