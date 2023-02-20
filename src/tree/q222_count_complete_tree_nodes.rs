use crate::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// type OptNode = Option<Rc<RefCell<TreeNode>>>;
// naive recursive
pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, cnt: &mut i32) {
        if let Some(node) = root {
            *cnt += 1;
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            dfs(left, cnt);
            dfs(right, cnt);
        }
    }

    let mut cnt = 0;
    dfs(&root, &mut cnt);
    cnt
}

pub fn count_nodes_3(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            1 + count_nodes_3(node.as_ref().borrow().right.clone())
                + count_nodes_3(node.as_ref().borrow().left.clone())
        }
    }
}
