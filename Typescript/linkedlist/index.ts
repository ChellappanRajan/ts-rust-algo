class LNode<T>{
    value:T;
    next:LNode<T> | null;
    constructor(value){
        this.value =value;
    }
}

class SinglyLinkedList{
    head:null | LNode<any> = null;
    tail:null | LNode<any>  = null;
    length = 0;

    push(value){
     const newNode = new LNode(value);
     if(!this.head){
        this.head = newNode;
     }
     if(!this.tail){
        this.tail = newNode
     }
     this.length = this.length +1;
    }
}

