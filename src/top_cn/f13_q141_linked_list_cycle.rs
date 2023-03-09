use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
pub struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    pub fn new(val: i32) -> ListNode {
        ListNode { val, next: None }
    }
}

pub fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> bool {
    let mut set = HashSet::new();

    let mut h = match head {
        Some(ref node) => Rc::clone(node),
        _ => return false,
    };

    loop {
        if set.contains(&Rc::as_ptr(&h)) {
            return true;
        }
        set.insert(Rc::as_ptr(&h));
        let next = match h.borrow().next {
            Some(ref node) => Rc::clone(node),
            _ => break,
        };
        h = next;
    }

    false
}

#[test]
fn cycle_work() {
    let node1 = Rc::new(RefCell::new(ListNode::new(1)));
    let node2 = Rc::new(RefCell::new(ListNode::new(2)));
    let node3 = Rc::new(RefCell::new(ListNode::new(3)));
    let node4 = Rc::new(RefCell::new(ListNode::new(4)));
    let node2_rc = Rc::clone(&node2);
    let node4_rc = Rc::clone(&node4);
    node3.borrow_mut().next = Some(node4);
    node2.borrow_mut().next = Some(node3);
    node1.borrow_mut().next = Some(node2);
    // node4_rc.borrow_mut().next = Some(node2_rc);
    node4_rc.borrow_mut().next = Some(node2_rc);

    let head = Some(node1);
    let ret = has_cycle(head);
    eprintln!("ret = {:?}", ret);
}
