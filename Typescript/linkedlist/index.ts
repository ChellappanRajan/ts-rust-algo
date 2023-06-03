class LNode<T>{
    value:T;
    next:LNode<T> | null;
    constructor(value){
        this.value =value;
    }
}

const node = new LNode(10);
const new_node = new LNode(20);
node.next.node = new_node;