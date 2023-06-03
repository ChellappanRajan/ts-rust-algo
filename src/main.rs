use std::rc::Rc;

struct Node<T>{
   value:T,
   next:Option<Rc<Node<T>>>
}

fn main() {
   let mut node = Node{
      value:10,
      next:None
   };
   let  new_node = Node{
      value:20,
      next:None
   };
   node.next = Some(Rc::new(new_node));
   print!("{}",node.next.unwrap().value);
}
