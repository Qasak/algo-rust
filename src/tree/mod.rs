use std::cell::RefCell;
use std::rc::Rc;

mod q1145_tree_winning_move;
mod q124_binary_tree_max_path_sum;
mod q124_max_path_sum;
pub mod q222_count_complete_tree_nodes;
mod q2331_evaluate_bool;
mod q872_leaf_similar;
mod q938_range_sum_bst;
mod q94_in_order;

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
            right: None,
        }
    }
}
