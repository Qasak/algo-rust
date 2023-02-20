use crate::linked_list::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // a: prev, b: curr, c: next
    let mut b = head;
    let mut a = None;
    while let Some(mut bb) = b {
        let c = bb.next.take();
        bb.next = a;
        a = Some(bb);
        b = c;
    }
    a
}

type OptNode = Option<Box<ListNode>>;
pub fn reverse_list_rec(head: OptNode) -> OptNode {
    // a: prev, b: curr, c: next
    fn reverse(a: OptNode, b: OptNode) -> OptNode {
        if let Some(mut bb) = b {
            let c = bb.next.take();
            bb.next = a;
            reverse(Some(bb), c)
        } else {
            a
        }
    }
    reverse(None, head)
}

#[test]
fn test() {
    let v = vec![1, 2, 3, 4, 5];
    let vv = v.clone();
    let list1: ListNode = vv.into();
    list1.print_list();
    let list: ListNode = v.into();
    let new_head = reverse_list(Some(Box::new(list)));
    new_head.unwrap().print_list();
}
