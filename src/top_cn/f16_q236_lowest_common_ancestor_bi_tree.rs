use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() || p == root || q == root {
        return root;
    }
    let l = lowest_common_ancestor(
        root.as_ref().unwrap().borrow_mut().left.take(),
        p.clone(),
        q.clone(),
    );
    let r = lowest_common_ancestor(
        root.as_ref().unwrap().borrow_mut().right.take(),
        p.clone(),
        q.clone(),
    );
    match (l, r) {
        (Some(_l), Some(_r)) => root,
        (Some(n), None) | (None, Some(n)) => Some(n),
        _ => None,
    }
}

#[test]
fn refcell_work() {
    struct TheDarkKnight;

    impl TheDarkKnight {
        fn nothing_is_true(self) {}
    }

    struct Batcave {
        knight: TheDarkKnight,
    }

    fn main() {
        use std::mem;

        let mut cave = Batcave {
            knight: TheDarkKnight,
        };
        let borrowed = &mut cave;

        // borrowed.knight.nothing_is_true(); // E0507
        mem::replace(&mut borrowed.knight, TheDarkKnight).nothing_is_true(); // ok!
    }
}
