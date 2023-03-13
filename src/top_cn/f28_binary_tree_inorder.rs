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