use std::cell::RefCell;
use std::rc::Rc;

pub mod q222_count_complete_tree_nodes;
mod q124_max_path_sum;
mod q938_range_sum_bst;
mod q872_leaf_similar;
mod q124_binary_tree_max_path_sum;
mod q1145_tree_winning_move;


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