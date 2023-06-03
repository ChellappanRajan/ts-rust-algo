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

node.node = new_node;

```

## Rust Example

In rust you can define a node using a struct:

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
-In Rust, Option is similar to null or a value of type T.
-To allow multiple values to point to a single variable, Rust uses a reference counter (Rc) wrapped inside an Option.