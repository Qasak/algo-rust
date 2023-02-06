use std::cell::RefCell;
use std::rc::Rc;
use crate::tree::TreeNode;

type OptNode = Option<Rc<RefCell<TreeNode>>>;
// post_order traverse
pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        let val = node.as_ref().unwrap().borrow().val;
        if val == 1 || val == 0 {
            val == 1
        } else {
            let l = dfs(&node.as_ref().unwrap().borrow().left);
            let r = dfs(&node.as_ref().unwrap().borrow().right);
            if val == 2 {
                l || r
            } else {
                l && r
            }
        }
    }
    dfs(&root)
}