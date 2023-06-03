use std::rc::Rc;

struct Node<T>{
   value:T,
   next:Option<Rc<Node<T>>>
}

impl<T> Node<T> {
    fn new(value:T)->Self{
      Self{
         value,
         next: None
      }
    }
}

fn main() {
   let mut node = Node::new(10);

   let mut new_node = Node::new(10);
   // let  new_node = Node{
   //    value:20,
   //    next:None
   // };
   node.next = Some(Rc::new(new_node));
   print!("{}",node.next.unwrap().value);
}
