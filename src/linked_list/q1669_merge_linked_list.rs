use std::ops::Deref;
use crate::linked_list::ListNode;

// pub fn merge_in_between(mut list1: Option<Box<ListNode>>, a: i32, b: i32, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let (a, b) = (a as usize, b as usize);
//     let mut v = vec![];
//     let mut head = &mut list1;
//     while head.is_some() {
//         v.push(head.as_mut().unwrap().val);
//         head = &mut head.as_mut().unwrap().next;
//     }
//     let mut d = Box::new(ListNode::new(0));
//     let mut p = &mut d;
//
//     for i in 0..a {
//         let mut cur = Box::new(ListNode::new(v[i]));
//         let nxt = &mut cur;
//         p.next = Some(cur);
//         p = nxt;
//     }
//
//     d.next
// }

// pub fn merge_in_between(mut list1: Option<Box<ListNode>>, a: i32, b: i32, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut start = &mut list1;
//     for _ in 1..a { start = &mut start.as_deref_mut().unwrap().next; }
//
//     let mut end = &mut start.clone();
//     for _ in a - 2..b { end = &mut end.as_deref_mut().unwrap().next; }
//     core::mem::swap(&mut start.as_deref_mut().unwrap().next, &mut list2);
//
//     while start.as_ref().unwrap().next.is_some() { start = &mut start.as_deref_mut().unwrap().next; }
//     core::mem::swap(&mut start.as_deref_mut().unwrap().next, &mut end);
//     list1
// }

use std::mem;
pub fn merge_in_between(mut list1: Option<Box<ListNode>>, a: i32, b: i32, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut d = &mut list1;

    let mut i = 1;
    while i < a {
        d = &mut d.as_mut().unwrap().next;
        i += 1;
    }
    // d指向a下标的前一个节点
    // p当前指向a下标的节点
    let mut p = d.as_mut().unwrap().next.take();
    while i <= b {
        p = p.as_mut().unwrap().next.take();
        i += 1;
    }
    // while跑完，p指向b+1下标的节点，中间的节点全部用None填充

    // q指向list2最后一个节点
    let mut q = &mut list2;
    while q.as_ref().unwrap().next.is_some() {
        q = &mut q.as_mut().unwrap().next;
    }

    q.as_mut().unwrap().next = p;
    d.as_mut().unwrap().next = list2;

    list1
}




fn show_list(head: Option<Box<ListNode>>) {
    let mut head = &head;
    while head.is_some() {
        print!("{}", head.as_ref().unwrap().val);
        head = &head.as_ref().unwrap().next;
        if head.is_some() {
            print!(" ");
        }
    }
}

#[test]
fn f() {
    let list1 = Some(Box::new(
        ListNode{ val: 0, next: Some(Box::new(
            ListNode{val: 1, next: Some(Box::new(
                ListNode{val: 2, next: Some(Box::new(
                    ListNode{val: 3, next: Some(Box::new(
                        ListNode{val: 4, next: Some(Box::new(
                            ListNode{val: 5, next: None
                            }))
                        }))
                    }))
                }))
            }))
        }));
    let list2 = Some(Box::new(
        ListNode{ val: 1000000, next: Some(Box::new(
            ListNode{val: 1000001, next: Some(Box::new(
                ListNode{val: 1000002, next: None
                }))
            }))
        }));
    // [0,1,2,1000000,1000001,1000002,5]
    show_list(merge_in_between(list1, 3, 4, list2));

}