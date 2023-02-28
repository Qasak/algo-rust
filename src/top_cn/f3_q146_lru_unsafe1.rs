use std::boxed::Box;
use std::collections::HashMap;
use std::ptr;

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
            prev: ptr::null_mut(), // create a null mutable raw pointer
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

struct LRUCache {
    map: HashMap<i32, Box<DListNode>>, // 结点由哈希表存储，形式为Box<LRUEntry>
    cap: usize,                       // 容量
    head: *mut DListNode,              // 头结点（哑）
    tail: *mut DListNode,              // 尾结点（哑）
}

impl LRUCache {
    fn new(cap: i32) -> Self {
        let mut init = Self {
            // map以Box<LRUEntry>的形式存储结点，而head和tail不存储于map中，所以需要手动管理
            map: HashMap::new(),
            cap: cap as usize,
            head: Box::into_raw(Box::new(DListNode::new_init())),
            tail: Box::into_raw(Box::new(DListNode::new_init())),
        };
        // 初始化链表连接性
        unsafe {
            (*init.head).next = init.tail;
            (*init.tail).prev = init.head;
        }
        init
    }

    // 将目标结点从链表中摘除，维护其前后结点的连接
    fn cut_out(&mut self, node: *mut DListNode) {
        unsafe {
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
        }
    }
    // 将目标结点连接至头结点后，维护相关结点的连接
    fn link_in(&mut self, node: *mut DListNode) {
        unsafe {
            (*node).next = (*self.head).next;
            (*self.head).next = node;
            (*node).prev = self.head;
            (*(*node).next).prev = node;
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        // 获取对应结点，并将智能指针转化为裸指针
        let entry = self
            .map
            .get_mut(&key)
            // DListNode的可变引用转成裸指针
            .map(|node| (&mut **node) as *mut DListNode);

        if let Some(entry) = entry {
            // 将对应结点更新至链表首位（头结点后）
            self.cut_out(entry);
            self.link_in(entry);

            unsafe { (*entry).val }
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, val: i32) {
        let entry = self
            .map
            .get_mut(&key)
            .map(|node| (&mut **node) as *mut DListNode); // 同fn get
        if let Some(entry) = entry {
            self.cut_out(entry);
            self.link_in(entry);
            unsafe {
                (*entry).val = val; // 更新值
            }
        } else {
            // 获取node（获取所有权并维护关系）
            let mut node = if self.map.len() == self.cap {
                let oldest_key = unsafe { (*(*self.tail).prev).key }; // 获得目标结点的键
                let mut oldest_entry = self.map.remove(&oldest_key).unwrap(); //  获取目标结点，所有权从map释放

                self.cut_out((&mut *oldest_entry) as *mut DListNode); // 解除该结点的联系

                // 更新结点: 此处可减少一次内存分配
                oldest_entry.key = key;
                oldest_entry.val = val;

                oldest_entry
            } else {
                Box::new(DListNode::new(key, val)) // 按存储形式构建新结点
            };

            self.link_in((&mut *node) as *mut DListNode); // 重新构建联系

            self.map.insert(key, node); // 处理完毕，所有权转交给map
  
        }
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

