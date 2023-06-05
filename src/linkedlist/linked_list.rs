use std::{borrow::Borrow, fmt, rc::Rc};

pub struct Node<T> {
    pub value: T,
    pub next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

pub struct SingleLinkedList<T> {
    pub head: Option<Rc<Node<T>>>,
    //pub tail:Option<Rc<Node<T>>>,
    pub length: usize,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        match self.head.take() {
            None => {
                let new_node = Rc::new(Node::new(value));
                self.head = Some(new_node);
                self.length = self.length + 1;
            }
            Some(node) => {
                let new_node = Rc::new(Node {
                    value,
                    next: Some(node),
                });
                self.head = Some(new_node);
                self.length = self.length + 1;
            }
        }
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next.clone();
            self.length = self.length - 1;
            Rc::try_unwrap(node).ok().unwrap().value
        })
    }
}
