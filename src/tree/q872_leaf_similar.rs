use crate::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type T = Option<Rc<RefCell<TreeNode>>>;

// iterative
pub fn leaf_similar(root1: T, root2: T) -> bool {
    iterative(root1) == iterative(root2)
}
fn iterative(root: T) -> Vec<i32> {
    let mut stack = vec![root];
    let mut leaf = vec![];

    while let Some(node) = stack.pop() {
        if let Some(node) = node {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                leaf.push(node.val);
            } else {
                stack.push(node.right.clone());
                stack.push(node.left.clone());
            }
        }
    }
    leaf
}

// recursive
pub fn leaf_similar_1(root1: T, root2: T) -> bool {
    fn get_leafs(root: &T, arr: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            if !node.left.is_some() && !node.right.is_some() {
                arr.push(node.val);
            } else {
                get_leafs(&node.left, arr);
                get_leafs(&node.right, arr);
            }
        }
    }
    let mut arr1 = vec![];
    let mut arr2 = vec![];
    get_leafs(&root1, &mut arr1);
    get_leafs(&root2, &mut arr2);
    arr1 == arr2
}
