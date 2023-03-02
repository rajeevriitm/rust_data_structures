// use std::rc::Rc;

// pub struct LinkedList<T> {
//     head: Option<Rc<Node<T>>>,
//     tail: Option<Rc<Node<T>>>,
// }

// struct Node<T> {
//     value: T,
//     next: Option<Rc<Node<T>>>,
// }

// impl<T> LinkedList<T> {
//     pub fn new() -> Self {
//         LinkedList {
//             head: None,
//             tail: None,
//         }
//     }

//     pub fn push(&mut self, value: T) {
//         let new_node = Rc::new(Node {
//             value,
//             next: self.head.take(),
//         });

//         self.head = Some(Rc::clone(&new_node));

//         if self.tail.is_none() {
//             self.tail = Some(Rc::clone(&new_node));
//         }
//     }

//     pub fn push_back(&mut self, value: T) {
//         let new_node = Rc::new(Node { value, next: None });

//         if let Some(tail) = self.tail.take() {
//             tail.next = Some(Rc::clone(&new_node));
//             self.tail = Some(Rc::clone(&new_node));
//         } else {
//             self.head = Some(Rc::clone(&new_node));
//             self.tail = Some(Rc::clone(&new_node));
//         }
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         self.head.take().map(|node| {
//             self.head = node.next.clone();

//             if self.head.is_none() {
//                 self.tail = None;
//             }

//             Rc::try_unwrap(node).ok().unwrap().value
//         })
//     }
// }
