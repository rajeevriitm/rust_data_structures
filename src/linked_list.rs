use std::{cell::RefCell, rc::Rc};

type NodeNext<T> = Option<Rc<RefCell<Node<T>>>>;
pub struct Linked_List<T> {
    head: NodeNext<T>,
    tail: NodeNext<T>,
    pub length: usize,
}
pub struct Node<T> {
    value: T,
    next: NodeNext<T>,
}
impl<T> Linked_List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn prepend(&mut self, val: T) {
        let node = Node {
            value: val,
            next: self.head.take(),
        };
        let node_rc = Rc::new(RefCell::new(node));
        self.head = Some(node_rc.clone());
        if self.tail.is_none() {
            self.tail = Some(node_rc);
        }
        self.length += 1;
    }
    pub fn append(&mut self, value: T) {
        let node = Node { value, next: None };
        let node_rc = Rc::new(RefCell::new(node));
        match self.tail.take() {
            Some(x) => x.borrow_mut().next = Some(node_rc.clone()),
            None => self.head = Some(node_rc.clone()),
        };
        self.tail = Some(node_rc);
        self.length += 1;
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if self.length == 1 {
            self.tail.take();
        }
        self.head.take().map(|x| {
            self.length -= 1;
            let val = Rc::try_unwrap(x)
                .ok()
                .expect("failed to Rc unwrap")
                .into_inner();
            self.head = val.next;
            val.value
        })
    }
}
