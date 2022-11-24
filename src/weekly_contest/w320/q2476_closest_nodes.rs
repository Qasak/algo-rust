use std::cell::RefCell;
use std::rc::Rc;
use crate::tree::q222_count_complete_tree_nodes::TreeNode;


// NAIVE search: TLE
// The question does not say that the BST is balanced;
// at worst the BST is a linked-list.
pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    fn floor(root: &Option<Rc<RefCell<TreeNode>>>, val: i32, pre: i32) -> i32 {

        if let Some(node) = &root.as_ref() {
            let cur = node.borrow().val;

            if cur > val {
                floor(&node.borrow().left, val, pre)
            } else if cur < val {
                floor(&node.borrow().right, val, cur)
            } else {
                val
            }
        } else if pre < val {
            pre
        } else {
            -1
        }
    }
    fn ceil(root: &Option<Rc<RefCell<TreeNode>>>, val: i32, pre: i32) -> i32 {
        if let Some(node) = &root.as_ref() {
            let cur = node.borrow().val;
            if cur < val {
                ceil(&node.borrow().right, val, pre)
            } else if cur > val {
                ceil(&node.borrow().left, val, cur)
            } else {
                val
            }
        } else if pre > val {
            pre
        } else {
            -1
        }
    }
    let cur = root.as_ref().unwrap().borrow().val;
    for q in queries {
        ret.push(vec![floor(&root, q, cur), ceil(&root, q, cur)]);
    }
    ret
}

// use binary_search
pub fn closest_nodes_1(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut arr = vec![];
    dfs(&root, &mut arr);
    let n = arr.len();
    for q in queries {
        ret.push(
            match arr.binary_search(&q) {
                Ok(_) => vec![q ,q],
                Err(0) => vec![-1, arr[0]],
                Err(i) => if i == n {vec![arr[n - 1], -1]} else {vec![arr[i - 1], arr[i]]}
            }
        );
    }
    ret
}

fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
    if let Some(node) = &root.as_ref() {
        let (l, r) = (&node.borrow().left, &node.borrow().right);
        dfs(l, arr);
        arr.push(node.borrow().val);
        dfs(r, arr);
    }
}
// manual bs
pub fn closest_nodes_2(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut arr = vec![];

    fn ceil(arr: &Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, arr.len());
        while l < r {
            let m = l + (r - l) / 2;
            if arr[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if l == arr.len() {
            -1
        } else {
            arr[l]
        }
    }
    fn floor(arr: &Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (-1, arr.len() as i32 - 1);
        while l < r {
            let m = l + (r - l + 1) / 2;
            if arr[m as usize] > target {
                r = m - 1;
            } else {
                l = m;
            }
        }
        if l == -1 {
            -1
        } else {
            arr[l as usize]
        }
    }
    dfs(&root, &mut arr);
    for q in queries {
        ret.push(vec![floor(&arr, q), ceil(&arr, q)]);
    }
    ret
}