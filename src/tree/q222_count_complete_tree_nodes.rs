use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

// naive recursive
pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, cnt: &mut i32) {
        if let Some(node) = root {
            *cnt += 1;
            let node = node.borrow();
            dfs(&node.left, cnt);
            dfs(&node.right, cnt);
        }
    }
    let mut cnt = 0;
    dfs(&root, &mut cnt);
    cnt
}