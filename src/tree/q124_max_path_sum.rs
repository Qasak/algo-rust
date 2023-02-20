use crate::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if let Some(node) = root.as_ref() {
            let node = node.borrow();
            let cur = node.val;
            let l = dfs(&node.left, ans);
            let r = dfs(&node.right, ans);
            let no_cross = cur + 0.max(l).max(r);
            let cross = cur + 0.max(l) + 0.max(r);
            *ans = (*ans).max(no_cross).max(cross);
            no_cross
        } else {
            0
        }
    }
    let mut ans = root.as_ref().unwrap().borrow().val;
    dfs(&root, &mut ans);
    ans
}
