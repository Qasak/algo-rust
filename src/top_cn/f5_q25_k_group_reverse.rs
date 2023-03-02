use crate::linked_list::ListNode;

type OptBoxNode = Option<Box<ListNode>>;

pub fn reverse_k_group(mut head: OptBoxNode, k: i32) -> OptBoxNode {
    // 定义一个辅助节点来指向头节点
    let mut cur = &head;
    // 计算链表长度

    // 如果链表长度小于k，则直接返回头节点
    for _ in 0..k {
        if let Some(node) = cur {
            cur = &node.next;
        } else {
            return head;
        }
    }

    // 定义一个辅助节点来指向头节点
    let mut cur = head;
    // 反转前k个节点，并将结果保存到变量prev中
    let mut prev = None;
    for _ in 0..k {
        if let Some(mut node) = cur {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            cur = next;
        }
    }

    // 递归反转剩余节点，并将结果保存到变量next中
    let next = reverse_k_group(cur, k);

    // 将前k个节点的最后一个节点指向后面节点的头部
    let mut last = prev.as_mut().unwrap();
    while let Some(ref mut node) = last.next {
        last = node;
    }
    last.next = next;
    // 返回反转后的头节点
    prev
}
