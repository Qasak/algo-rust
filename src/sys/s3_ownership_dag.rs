use std::rc::Rc;

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

#[cfg(test)]
mod test {
    use crate::sys::s3_ownership_dag::Node;
    use std::rc::Rc;

    #[test]
    fn node_test() {
        //     1  2
        //     | /
        //     3
        //     |
        //     4
        let mut node1 = Node::new(1);
        let mut node2 = Node::new(2);
        let mut node3 = Node::new(3);
        let node4 = Node::new(4);
        node3.update_downstream(Rc::new(node4));

        node1.update_downstream(Rc::new(node3));
        node2.update_downstream(node1.get_downstream().unwrap());
        println!("node1: {:?}, node2: {:?}", node1, node2);
    }

    #[test]
    fn refcell_test() {
        use std::cell::RefCell;

        let data = RefCell::new(1);
        {
            // 获得 RefCell 内部数据的可变借用
            let mut v = data.borrow_mut();
            *v += 1;
        }
        println!("data: {:?}", data.borrow());
    }

    #[test]
    fn strong_count_work() {
        use std::rc::Rc;

        #[derive(Debug)]
        struct Point {
            x: f64,
            y: f64,
        }

        let p1 = Rc::new(Point { x: 1.0, y: 2.0 });
        let p2 = p1.clone();

        let p_ref1 = &*p1;
        let p_ref2 = p1.clone();

        println!("p1: {:?}, refcount: {}", p1, Rc::strong_count(&p1));
        println!("p2: {:?}, refcount: {}", p2, Rc::strong_count(&p2));
        println!("p_ref1: {:?}", p_ref1);
        println!("p_ref2: {:?}", p_ref2);
    }
}
