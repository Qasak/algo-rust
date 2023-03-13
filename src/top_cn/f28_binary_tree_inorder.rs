use std::{rc::Rc, cell::RefCell};

use crate::tree::TreeNode;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn in_order_dfs(root: &Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>) {
        if let Some(node) = root {
            in_order_dfs(&node.borrow().left, path);
            path.push(node.borrow().val);
            in_order_dfs(&node.borrow().right, path);
        }
    }
    let mut ret = vec![];
    in_order_dfs(&root, &mut ret);
    ret
}

pub fn inorder_traversal_iter(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // stk: Vec<Rc<RefCell<TreeNode>>>
    let mut stk = vec![];
    let mut ret = vec![];
    let mut cur = root.take();
    while !stk.is_empty() || cur.is_some() {
        // Rc
        while let Some(node) = cur {
            cur = node.borrow_mut().left.take();
            stk.push(node);
        };
        // Rc
        if let Some(node) = stk.pop() {
            ret.push(node.borrow().val);
            cur = node.borrow_mut().right.take();
        };
    }
    ret
}