use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }
    use std::collections::VecDeque;
    let mut ret = vec![];
    let mut dq = VecDeque::new();
    dq.push_back(root.unwrap());
    while !dq.is_empty() {
        let mut level = vec![];
        for _ in 0..dq.len() {
            let cur = dq.pop_front().unwrap();
            level.push(cur.borrow().val);
            if cur.borrow().left.is_some() {
                dq.push_back(cur.borrow().left.as_ref().unwrap().clone());
            }
            if cur.borrow().right.is_some() {
                dq.push_back(cur.borrow().right.as_ref().unwrap().clone());
            }
        }
        ret.push(level);
    }
    ret
}
