use std::{collections::HashMap, ptr};
struct LRUCache {
    map: HashMap<i32, Box<DListNode>>,
    cap: usize,
    head: *mut DListNode,
    tail: *mut DListNode,
}

struct DListNode {
    key: i32,
    val: i32,
    prev: *mut DListNode,
    next: *mut DListNode,
}

impl DListNode {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
    
    fn new_init() -> Self {
        Self {
            key: 0,
            val: 0,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let mut lru = Self {
            map: HashMap::new(),
            cap: capacity as usize,
            head: Box::into_raw(Box::new(DListNode::new_init())),
            tail: Box::into_raw(Box::new(DListNode::new_init())),
        };
        unsafe {
            (*lru.head).next = lru.tail;
            (*lru.tail).prev = lru.head;
        }
        lru
    }

    fn link_in(&mut self, node: *mut DListNode) {
        unsafe {
            (*node).next = (*self.head).next;
            (*(*self.head).next).prev = node;
            (*self.head).next = node;
            (*node).prev = self.head;
        }
    }

    // 何时内存回收？
    fn cut_off(&mut self, node: *mut DListNode) {
        unsafe {
            (*(*node).next).prev = (*node).prev;
            (*(*node).prev).next = (*node).next;
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        let mut entry = self
            .map
            .get_mut(&key)
            .map(|node| (&mut **node) as *mut DListNode);
        if let Some(mut node) = entry {
            self.cut_off(node);
            self.link_in(node);
            unsafe {
                (*node).val
            }
        } else {
            -1
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        let mut entry = self
            .map
            .get_mut(&key)
            .map(|node| (&mut **node) as *mut DListNode);
        //存在key, 修改value即可
        if let Some(mut old_node_entry) = entry  {
            self.cut_off(old_node_entry);
            self.link_in(old_node_entry);
            unsafe {
                (*old_node_entry).val = value;
            }
        //加入一个新key，首先判断是否满cap
        } else {
            // 找到老节点并更新值
            if self.map.len() == self.cap {
                let old_key = unsafe{(*(*self.tail).prev).key};
                // 通过remove获得旧节点
                let mut old_node = self.map.remove(&old_key).unwrap();
                old_node.key = key;
                old_node.val = value;
                self.cut_off((&mut *old_node) as *mut DListNode);
                self.link_in((&mut *old_node) as *mut DListNode);
                self.map.insert(key, old_node);
            } else {
                let mut node = Box::new(DListNode::new(key, value));
                self.link_in((&mut *node) as *mut DListNode);
                self.map.insert(key, node);
            }


            // 获取node（获取所有权并维护关系）
            // let mut node = if self.map.len() == self.cap {
            //     let oldest_key = unsafe { (*(*self.tail).prev).key }; // 获得目标结点的键
            //     let mut oldest_entry = self.map.remove(&oldest_key).unwrap(); //  获取目标结点，所有权从map释放

            //     self.cut_off((&mut *oldest_entry) as *mut DListNode); // 解除该结点的联系

            //     // 更新结点: 此处可减少一次内存分配
            //     oldest_entry.key = key;
            //     oldest_entry.val = value;

            //     oldest_entry
            // } else {
            //     Box::new(DListNode::new(key, value)) // 按存储形式构建新结点
            // };

            // self.link_in((&mut *node) as *mut DListNode); // 重新构建联系

            // self.map.insert(key, node); // 处理完毕，所有权转交给map
        };
    }
}

#[test]
fn lru_work() {
    let mut lru = LRUCache::new(2);
    // ["LRUCache","put","put","get","put","get","put","get","get","get"]
    // [[2],[1,0],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]
    lru.put(1, 2);
    lru.put(2, 2);
    lru.get(1);
    lru.put(3, 3);
    lru.get(2);
    lru.put(4, 4);
    lru.get(1);
    lru.get(3);
    lru.get(4);
}