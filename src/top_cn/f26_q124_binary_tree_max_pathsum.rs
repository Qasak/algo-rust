use std::{rc::Rc, cell::RefCell};

use crate::tree::TreeNode;

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn get_subtree_contribute(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if let Some(ref node) = root {
            let node = node.borrow();
            let cur = node.val;
            let l = get_subtree_contribute(&node.left, ans);
            let r = get_subtree_contribute(&node.right, ans);
            let cur_contrib = cur + 0.max(l).max(r);
            let cross = cur + 0.max(l) + 0.max(r);
            // 1. not select current node
            // 2. select current node + prev nodes
            // 3. select current node
            *ans = (*ans).max(cur_contrib).max(cross);
            cur_contrib
        } else {0}
    }
    let mut ans = root.as_ref().unwrap().borrow().val;
    get_subtree_contribute(&root, &mut ans);
    ans
}