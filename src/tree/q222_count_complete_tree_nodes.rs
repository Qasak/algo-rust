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

// naive solution 2
pub fn count_nodes_1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cnt: &mut i32) {
        if let Some(node) = root {
            *cnt += 1;
            let mut node = node.borrow_mut();
            dfs(node.left.take(), cnt);
            dfs(node.right.take(), cnt);
        }
    }
    let mut cnt = 0;
    dfs(root, &mut cnt);
    cnt
}

// naive solution 3
pub fn count_nodes_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: &Rc<RefCell<TreeNode>>) -> i32 {
        1 + if let Some(v) = &root.borrow().left {
            dfs(&v)
        } else {0}
            + if let Some(v) = &root.borrow().right {
            dfs(&v)
        } else {0}
    }
    if let Some(v) = root {
        dfs(&v)
    } else {0}
}

