use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::tree::TreeNode;

type OptNode = Option<Rc<RefCell<TreeNode>>>;
// recursive
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = vec![];
    fn dfs(node: &OptNode, ret: &mut Vec<i32>) {
        if let Some(n) = node.as_ref() {
            let n = n.borrow();
            let val = n.val;
            dfs(&n.left, ret);
            ret.push(val);
            dfs(&n.right, ret);
        }
    }
    dfs(&root, &mut ret);
    ret
}

// non-recursive
// 两种处理：
// 1. 把树整个clone
// 2. 把树整个take
// 如果想用容器保存RefCell的borrow返回的Ref是不行的，因为Ref会在下一次while循环被drop
// 例如：
// let x = &cur.as_ref().unwrap().borrow().left;
// x: &Option<Rc<RefCell<TreeNode>>>
//
// pub fn inorder_traversal_temporary_ref_dropped(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
//     let mut ret = vec![];
//     let mut stk = VecDeque::new();
//     let mut cur = &root;
//     while !stk.is_empty() || !cur.is_none() {
//         while !cur.is_none() {
//             stk.push_back(cur);
//             let x= &cur.as_ref().unwrap().borrow().left;
//             cur = &cur.as_ref().unwrap().borrow().left;
//         }
//         cur = stk.pop_back().unwrap();
//         ret.push(cur.as_ref().unwrap().borrow().val);
//         cur = &cur.as_ref().unwrap().borrow().right;
//     }
//     ret
// }

// Option的clone： 会调用内部类型的clone
// 那么在这里会调用Rc的clone，增加一次引用计数
//     fn clone(&self) -> Self {
//         match self {
//             Some(x) => Some(x.clone()),
//             None => None,
//         }
//     }

// 1. 把树整个clone
pub fn inorder_traversal_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = vec![];
    let mut stk = VecDeque::new();
    let mut cur = root;
    while !stk.is_empty() || !cur.is_none() {
        // n: Rc
        while let Some(n) = cur {
            stk.push_back(n.clone());
            cur = n.borrow().left.clone();
        }
        // n: Rc
        if let Some(n) = stk.pop_back() {
            ret.push(n.borrow().val);
            cur = n.borrow().right.clone();
        }
    }
    ret
}

// 2. 把树整个take
pub fn inorder_traversal_2(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = vec![];
    let mut stk = VecDeque::new();
    let mut cur = root.take();
    while !stk.is_empty() || !cur.is_none() {
        // n: Rc
        while let Some(n) = cur {
            cur = n.borrow_mut().left.take();
            stk.push_back(n);
        }
        // n: Rc
        if let Some(n) = stk.pop_back() {
            cur = n.borrow_mut().right.take();
            ret.push(n.borrow_mut().val);
        }
    }
    ret
}