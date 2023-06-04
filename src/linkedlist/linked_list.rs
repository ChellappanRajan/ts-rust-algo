use std::{rc::Rc, fmt};

pub struct Node<T>{
   pub value:T,
   pub next:Option<Rc<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(value:T)->Self{
      Self{
         value,
         next: None
      }
    }
}

pub struct SingleLinkedList<T> {
    pub head:Option<Rc<Node<T>>>,
    pub tail:Option<Rc<Node<T>>>,
    pub length:usize
}

impl<T> SingleLinkedList<T> {
    pub fn new()->Self{
      Self { head: None, tail: None, length: 0 }
   }

   // pub fn push(&mut self,value: T){
   //    let new_node = Node::new(value);
      
   //    if let Some(_) = &self.tail {
   //       // self.tail.clone().unwrap().next = Some(Rc::new(new_node));
   //    } else {
   //       self.head = Some(Rc::new(new_node));
   //       self.tail = self.head.clone();
   //    }
   // }
   pub fn push(&mut self, value: T) {
   let new_node = Rc::new(Node::new(value));

   if let Some(mut node) = self.tail.clone() {
      node = new_node.clone();
      self.tail = Some(new_node);
   } else {
       self.head = Some(new_node);
       self.tail = self.head.clone();
   }
}

// fn add_last(&mut self, value: T) {
//    let new_node = Rc::new(Node {
//        value,
//        next: None,
//    });

//    if let Some(mut node) = self.head.as_mut() {
//        while let Some(next_node) = node.next.as_ref() {
//            node = next_node.clone();
//        }
//        node.next = Some(new_node);
//    } else {
//        self.head = Some(new_node);
//    }
// }


}





// impl<T> fmt::Display for SingleLinkedList<T> {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "({:?}", self.head.unwrap().value)
//    }
// }