use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;
    if root.is_none() {
        return vec![];
    }
    let mut ret = vec![];
    let mut cnt = 0;
    let mut dq = VecDeque::new();
    dq.push_back(root.unwrap());
    while !dq.is_empty() {
        let len = dq.len();
        let mut tmp = vec![];
        for _ in 0..len {
            let node = dq.pop_front().unwrap();
            tmp.push(node.borrow().val);
            if let Some(ref left) = node.borrow().left {
                dq.push_back(left.clone());
            };
            if let Some(ref right) = node.borrow().right {
                dq.push_back(right.clone());
            // 加上分号";"让临时变量在这里被drop
            };
        }
        if cnt % 2 == 1 {
            tmp.reverse();
        }
        ret.push(tmp);
        cnt += 1;
    }
    ret
}

#[test]
fn reverse_work() {
    let mut v = vec![1,2,3];
    v.reverse();
    eprintln!("v = {:?}", v);
}

#[test]
fn borrow_work() {
    let c = RefCell::new(5);
    let borrowed_five = c.borrow();
    let borrowed_five2 = c.borrow();

    eprintln!("borrowed_five = {:?}", borrowed_five);
    eprintln!("borrowed_five2 = {:?}", borrowed_five2);

}