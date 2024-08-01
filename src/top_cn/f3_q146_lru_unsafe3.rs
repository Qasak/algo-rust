use std::collections::HashMap;
use std::ptr;
use std::boxed::Box;

struct LRUCache {
    map: HashMap<i32, *mut Node>,
    cap: usize,
    head: *mut Node,
    tail: *mut Node,
}

struct Node {
    key: i32,
    val: i32,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            prev: ptr::null_mut(),
            next: ptr::null_mut()
        }
    }

    fn new_init() -> Self {
        Self {
            key: 0,
            val: 0,
            prev: ptr::null_mut(),
            next: ptr::null_mut()
        }
    }
    // 清理节点及其所有后续节点  
    fn free(node: *mut Node) {  
        if node.is_null() {  
            return;  
        }  
        println!("free node: {:?}", unsafe{(*node).key});
        drop(unsafe { Box::from_raw(node) }); // 将裸指针转换为Box，然后drop释放内存  
    }  
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    // 假设这是LRUCache的析构函数或清理函数  
    fn cleanup(&mut self) {  
        // 遍历哈希表并删除所有条目  
        // 注意：这取决于你使用的哈希表实现，这里只是一个伪代码  
        for (_, node_ptr) in self.map.drain() {  
            Node::free(node_ptr);  
        }  
        // for (_, node_ptr) in self.map.drain() {  
        //     println!("free node: {:?}", unsafe{(*node_ptr).key});
        // }  
        // 如果头尾哨兵节点是动态分配的，也需要释放它们  
        // 但在这个例子中，我们假设它们是栈分配的或全局的  
  
        // 重置头尾指针为null（可选）  
        self.head = ptr::null_mut();  
        self.tail = ptr::null_mut();  
    }  
    fn new(capacity: i32) -> Self {
        let map = HashMap::new();
        let head = Box::into_raw(Box::new(Node::new_init()));
        let tail = Box::into_raw(Box::new(Node::new_init()));
        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }
        Self {
            map,
            cap: capacity as usize,
            head,
            tail
        }
    }
    // 从双向链表取下
    fn cut(&mut self, node: *mut Node) {
        unsafe {
            (*(*node).next).prev = (*node).prev;
            (*(*node).prev).next = (*node).next;
            (*node).next = ptr::null_mut();
            (*node).prev = ptr::null_mut();
        }
    }
    // 将没有连接的Node移动到链表头
    fn move_to_head(&mut self, node: *mut Node) {
        unsafe {
            (*node).next = (*self.head).next;
            (*node).prev = self.head;
            (*(*self.head).next).prev = node;
            (*self.head).next = node;
        }
    }
    // 访问entry, 并将node放到链表头
    fn get(&mut self, key: i32) -> i32 {
        if let Some( node) = self.map.get(&key) {
            let node = *node;
            self.cut(node);
            self.move_to_head(node);
            // 访问也需要unsafe
            unsafe {
                (*node).val
            }
        } else {
            -1
        }
    }
    // 1.存在：1）修改 2）放到链表头
    // 2.不存在：1）满：取末尾entry Node，修改后放到链表头 2）不满：新建Node，放到链表头
    fn put(&mut self, key: i32, value: i32) {
        // 1.存在：1）修改 2）放到链表头
        if let Some(node) = self.map.get(&key) {
            let node = *node;
            self.cut(node);
            self.move_to_head(node);
            unsafe {
                (*node).val = value;
            }
        // 2.不存在：1）满：取末尾entry Node，修改后放到链表头 3)map删除entry Node. 2）不满：新建Node，放到链表头
        } else {
            if self.map.len() == self.cap {
                // println!("self.map.len(){}, self.cap:{}, key: {}", self.map.len(), self.cap, key);
                println!("full, key: {}", key);
                let node = unsafe{(*(self.tail)).prev};
                self.cut(node);
                self.move_to_head(node);
                self.map.remove(&unsafe{(*node).key});
                unsafe {
                    (*node).val = value;
                    (*node).key = key;
                }
                self.map.insert(key, node);
            } else {
                let node = Box::into_raw(Box::new(Node::new(key, value)));
                self.move_to_head(node);
                unsafe {
                    (*node).val = value;
                    (*node).key = key;
                }
                self.map.insert(key, node);
            }
        }
    }
}

#[test]
fn lru_work() {
    let mut lru = LRUCache::new(2);
    // ["LRUCache","put","put","get","put","get","put","get","get","get"]
    // [[2],     [1,0],  [2,2] ,[1], [3,3], [2], [4,4], [1], [3],  [4]]
    // (1, -1, -1, 3, 4)
    lru.put(1, 1);
    lru.put(2, 2);
    let ret1 = lru.get(1);
    lru.put(3, 3);
    let ret2 = lru.get(2);
    lru.put(4, 4);
    let ret3 = lru.get(1);
    let ret4 = lru.get(3);
    let ret5 = lru.get(4);
    println!("{:?}", (ret1, ret2, ret3, ret4, ret5));
    lru.cleanup();

}

#[test]
fn lru_work_2() {
    let mut lru = LRUCache::new(1);
    // ["LRUCache","put","put","get","put","get","put","get","get","get"]
    // [[1],     [1,0],  [2,2] ,[1], [3,3], [2], [4,4], [1], [3],  [4]]
    // (-1, -1, -1, -1, 4)
    lru.put(1, 1);
    lru.put(2, 2);
    let ret1 = lru.get(1);
    lru.put(3, 3);
    let ret2 = lru.get(2);
    lru.put(4, 4);
    let ret3 = lru.get(1);
    let ret4 = lru.get(3);
    let ret5 = lru.get(4);
    println!("{:?}", (ret1, ret2, ret3, ret4, ret5));
    lru.cleanup();
}

#[test]
fn lru_work_3() {
    let mut lru = LRUCache::new(5);
    // ["LRUCache","put","put","get","put","get","put","get","get","get"]
    // [[1],     [1,0],  [2,2] ,[1], [3,3], [2], [4,4], [1], [3],  [4]]
    // (-1, -1, -1, -1, 4)
    lru.put(1, 1);
    lru.put(2, 2);
    let ret1 = lru.get(1);
    lru.put(3, 3);
    let ret2 = lru.get(2);
    lru.put(4, 4);
    let ret3 = lru.get(1);
    let ret4 = lru.get(3);
    let ret5 = lru.get(4);
    println!("{:?}", (ret1, ret2, ret3, ret4, ret5));
    lru.cleanup();
}