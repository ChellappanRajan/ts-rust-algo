class SingleNode<T>{
    value:T;
    next:SingleNode<T> | null=null;
    constructor(value){
        this.value =value;
    }
}

class SingleLinkedList{
    head:null | SingleNode<any> = null;
    tail:null | SingleNode<any>  = null;
    length = 0;

    push(value){
     const newNode = new SingleNode(value);
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

    insertFirst(value){
    const newNode = new SingleNode(value);
    if(!this.head){
        this.head =newNode; 
        this.tail = this.head;
    }
    const oldHeadNode = this.head;
    this.head = newNode;
    this.head.next = oldHeadNode;
    this.tail = oldHeadNode;
    this.length++;
    }

    display():void{
        let temp = this.head;
        for(let i=1;i<= this.length;i++){
            console.log(temp?.value);
            temp = temp?.next!;
        }
    }
    
    insertLast(value){
        if(!this.tail){
            this.insertFirst(value);
            return;
        }
        const newNode = new SingleNode(value);
        const oldTailNode = this.tail;
        oldTailNode.next = newNode;
        this.tail =  newNode;
        this.length++;

    }

    get(index:number):number{
        let temp = this.head!;
        for(let i =1;i<index;i++){
            temp = temp?.next!;
        }
        return temp.value;
    }       
}

const single = new SingleLinkedList();

single.push(1);
single.push(2);
single.push(3);
single.push(4);
single.push(5);
console.log(JSON.stringify(single,null));

