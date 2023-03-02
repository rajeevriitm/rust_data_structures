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
        self.tail.as_ref().map(|x| {
            x.borrow_mut().next = Some(node_rc.clone());
        });
        if self.head.is_none() {
            self.head = Some(node_rc.clone())
        }
        self.tail = Some(node_rc);
        self.length += 1;
    }
}
