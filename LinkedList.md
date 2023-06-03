# Linked List
  Linked list is a data structure that stores data in a linear squence.     
  - The data in Linked list stored in nodes.
  - A data structure that contain `head` `tail` and length property. Head will points to first node whereas tail will point to last.
  - Linked list consits of nodes, each node have value and pointer to next node.
 
## JavaScript

Node is simple it stores piece of data we call it value and it stores reference to next node.

### JavaScript Example

you can create a simple node object in javascript like this:

```javascript
const node = {
    value:1,
    next:null
};
```

Alternatively, you can define a node class in TypeScript like this:

```typescript
class LNode<T>{
    value:T;
    next:LNode<T> | null;
    constructor(value){
        this.value =value;
    }
}

```

you can link nodes together by assiging next property with another node object as shown in the following example:

```javascript
const new_node = {
    value:1,
    next:null
};

node.next = new_node;
node.next.next = new_node;

```

However this approach becomes difficult when adding more nodes,as you would need to keep writing `node.next.next..` to access the desired node.

To improve this, you can create a `SinglyLinkedList` class and move adding node logic to it.    

```javascript
    class SinglyLinkedList{
       constructor(){
         this.head =null;
         this.tail = null;
         this.length = 0;
       }
    }

    const list = new SinglyLinkedList();
```
 Linked list will have the pointer `head` and `tail` as well as the `length` of the list.

 Now let's implement push method to add nodes:

 ```javascript
     push(value){
       const new_node = {
        value, 
        next:null
       }
       if(!this.head){
        this.head = new_node;
        this.tail = new_node;
       }else{
        this.tail.next = new_node;
       }
     }

    list.push("Helo");
    list.push("World");
 ```
 Everytime when we call `push` we create a `new_node` with the value we passed.If there is no head value, we set the `new_node` as the head and tail and     then increment the length by one.If there is already value then we create a `new_node` then assign it to tail node.



## Rust Example

In rust you can define a node using a struct:
- In Rust, Option is similar to null or a value of type T.
- To allow multiple values to point to a single variable, Rust uses a reference counter (Rc) wrapped inside an Option.


```rust 
struct Node<T>{
   value:T,
   next:Option<Rc<Node<T>>>
}
```

To modify the data in rust, you need to use `mut` for mutable variables.

Here's an example:

```rust
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
```

You can also define methods for the Node struct using the `impl` block:

```rust
impl<T> Node<T> {
    // This `impl` block defines the methods for the `Node` struct.
    fn new(value:T)->Self{
        // This `fn` method constructs a new `Node` instance.
      Self{
         // This `Self` struct literal initializes the `value` and `next` fields.
         value,
         next: None
      }
    }
}
```
The `new()` method constructs a new `Node` instance and initializes the value and next fields.

Refactored version of the above code:

```rust
   let mut node = Node::new(10);
   let  new_node = Node::new(20);
   node.next = Some(Rc::new(new_node));
   print!("{}",node.next.unwrap().value);
```




