use std::rc::Rc;

use algo_and_ds::linkedlist::linked_list::{Node, SingleLinkedList};


fn main() {
   // let mut node = Node::new(10);

   // let mut new_node = Node::new(20);
   // // let  new_node = Node{
   // //    value:20,
   // //    next:None
   // // };
   // node.next = Some(Rc::new(new_node));
   // print!("{}",node.next.unwrap().value);


   let mut list = SingleLinkedList::new();
   list.push(10);
   list.push(20);
   list.push(20);
   list.push(20);
   list.pop();

   
   print!("Length: {:?}",list.length);
   
}

