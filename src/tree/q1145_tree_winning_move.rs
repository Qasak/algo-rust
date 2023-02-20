use crate::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
type OptNode = Option<Rc<RefCell<TreeNode>>>;

pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
    fn dfs(node: &OptNode, x: i32, lsz: &mut i32, rsz: &mut i32) -> i32 {
        if !node.is_some() {
            return 0;
        }
        let l = dfs(&node.as_ref().unwrap().borrow().left, x, lsz, rsz);
        let r = dfs(&node.as_ref().unwrap().borrow().right, x, lsz, rsz);
        if node.as_ref().unwrap().borrow().val == x {
            *lsz = l;
            *rsz = r;
        }
        1 + l + r
    }

    let (mut lsz, mut rsz) = (0, 0);
    dfs(&root, x, &mut lsz, &mut rsz);
    let n2 = lsz.max(rsz.max(n - 1 - lsz - rsz));
    let n1 = n - n2;
    n2 > n1
}
