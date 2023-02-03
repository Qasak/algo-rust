use std::rc::Rc;
use std::cell::RefCell;
use crate::tree::TreeNode;

type T = Option<Rc<RefCell<TreeNode>>>;


// straight dfs
pub fn range_sum_bst(root: T, low: i32, high: i32) -> i32 {
    fn dfs(root: &T, low: i32, high: i32) -> i32{
        if let Some(node) = root {
            let node = node.borrow();
            let cur = node.val;
            let a = if cur < low || cur > high {0} else {cur};
            let b = if cur == high {0} else {dfs(&node.right, low, high)};
            let c = if cur == low {0} else {dfs(&node.left, low, high)};
            a + b + c
        } else {
            0
        }
    }
    dfs(&root, low, high)
}


