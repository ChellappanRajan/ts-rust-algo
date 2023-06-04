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
     if(!this.head || !this.tail){
        this.head = newNode;
        this.tail = newNode;
     }
     else{
        this.tail.next = newNode;
        this.tail = newNode;
     }
     this.length++;
     return this;
    }
}

