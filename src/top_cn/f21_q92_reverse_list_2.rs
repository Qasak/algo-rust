use crate::linked_list::ListNode;

pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
        // dummy头部
        let mut dummy = ListNode{next:None, val:-1};
        dummy.next = head;
        let mut ptr = &mut dummy;
        for _ in 0..left - 1 {
            // Option<Box<ListNode>>.as_mut().unwrap()
            ptr = ptr.next.as_mut().unwrap();
        }
        // ptr = dummy 指向left - 1, 令cur 指向left 开始反转
        // Option.take() : mem::replace(self, None) 在不破坏所有权规则的情况下交换或替换值
        // 不使用.take()会出现 cannot move out of `ptr.next` which is behind a mutable reference
        let mut curr = ptr.next.take();
        let mut prev = None;
        for _ in left..=right {
            if let Some(mut node) = curr {
                // node 不是一个可变引用，所以不会出现上面let mut curr = ptr.next不使用take()出现的错误
                let next = node.next;
                node.next = prev;
                prev = Some(node);
                curr = next;
            }
        }
        // 当前curr 指向right + 1
        // 当前prev 指向right
        // 令left - 1指向right
        ptr.next = prev;
        // 先令ptr指向right
        // 再令ptr指向curr
        for _ in left..=right {
            ptr = ptr.next.as_mut().unwrap();
        }
        ptr.next = curr;
        // 此时dummy依然指向head(ptr的值是复制的，ptr的修改不影响dummy的值)
        dummy.next
}
