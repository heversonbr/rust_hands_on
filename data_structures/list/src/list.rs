
// Box<T> : Smart Pointer used for allocating values on the heap 
//          it implements the Deref trait, which allows Box<T> values to be treated like references.
//          it implemetns the Drop trait, which allows to clean the the heap data that the box is pointing to when a Box<T> value goes out of scope
//          In this case we use the Box to make it possible to allow allocating a fixed size of the heap to store the pointer, 
//          instead of an unknown size required by the generic T type.   
//          REF: https://doc.rust-lang.org/book/ch15-01-box.html     

// Option: A special kind of enum. enums give you a way of saying a value is one of a possible set of values. 
//         The Option type encodes the very common scenario in which a value could be 'something' or it could be 'nothing'.
//         We can use Option to represent a value that can be 'NULL' (None).
//         In this case, we use Option to tell the compiler that the 'next' value can be either a 'pointer' to a node, or 'nothing' (none). 
// 
//         pub enum Option<T> {
//            None,
//            Some(T),
//         } 
//
// #[derive(Debug)] The println! macro can do many kinds of formatting, and by default, the curly brackets tell println! to use formatting known as Display
//                  structs don’t have a provided implementation of Display to use with println! and the {} placeholder. 
//                  trying to print structs will end up in an error => error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
//                  to avoid this error we have either to implement the Display and Debug traits to our struct 
//                                      or (easier solution) we add the outer attribute #[derive(Debug)] just before the struct definition
//                  REF: https://doc.rust-lang.org/book/ch05-02-example-structs.html#adding-useful-functionality-with-derived-traits
#[allow(dead_code)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
} 

#[allow(dead_code)]
#[derive(Debug)]
pub struct SinglyLinkedList<T>{
    /// The 'head' value represents the head of our list, 
    /// it is a pointer that references a the first node in the list 
    /// so it is either a reference to a node of typer T (&Node<T>) or it is a None (null pointer)
    head: Option<Box<Node<T>>>,
    lenght: usize
}

#[allow(dead_code)]
impl<T> Node<T> {
    /// Function 'new' receives a type T and returns a new structure of 
    /// type Node (with data type T)
    fn new(newdata: T) -> Self  {
        Node {
            data : newdata,
            next : None  // initially references a None, which will be set later
        }
    }
    
    fn set_next(&mut self, next_node: Node<T>){
        self.next = Some(Box::new(next_node));
    }
}



#[allow(dead_code)]
impl<T: std::cmp::PartialEq + std::fmt::Debug> SinglyLinkedList<T> {
    /// Function 'new' returns a new instance of the synglylinkedlist
    pub fn new() -> Self {
       SinglyLinkedList {
           head: None,
           lenght: 0
       }  
    }

    pub fn len(&self) -> usize { 
        self.lenght
    }

    // In our list a node can be added in three ways: 
    //   1) at the front of the linked list  
    //   2) after a given node. 
    //   3) at the end of the linked list.

    pub fn insert_front(&mut self, element: T) {
        // This function adds a new Node of type T in the front of the list
        // Arguments: receives the value of type T corresponding to the value of the new node
        print!("Inserting value [{:?}] in front of the list\n", element);
        let mut newnode = Node::new(element);    
        // function new returns a new Node with value of type T
        //println!("Created: {:#?}" , &newnode);

        //head empty:  empty list
        if self.head.is_none() {                
           //println!("Head Null");
           self.head = Some(Box::new(newnode));  
           //println!("lenght: {}" , &self.lenght); 
           //println!("self.head: {:?} ", &self.head);  
        }else{
            // list not empty: new node becomes head, 
            // head is attached to the next value of the new node

            //println!("Head NOT Null: {:?} ", &self.head);
            // take() method: Takes the value out of the option, leaving a None in its place.
            newnode.next = self.head.take();    
            //print_type_of(&self.head);
            //print_type_of(&newnode.next);
            self.head = Some(Box::new(newnode));
        }
        // just increases the length of the list
        self.lenght+=1;  
    }

    pub fn show(&self){
        //let mut current = &self.head;  // self.head ->  Option<Box<Node<T>>>
        //println!("Head: {:?}",  current.as_ref().unwrap());   
        //while current.is_some() {
        //    println!("current: {:?} ", current);
        //    println!("Node: {:?} , Next : {:?}", current.as_ref().unwrap().data, &current.as_ref().unwrap().next);
        //    current = &(current.as_ref().unwrap().next);
        //}
        
        println!("Print list:");
        if self.head.is_some(){  // list in NOT empty:  head has an Option that is a ref to a Node: &Node

            let mut current_node = &self.head;  // self.head` has type `Option<Box<Node<T>>> , I pass a ref to head to avoid moving the ownership 
                                                //  then current might be a &Option that is a ref (Box) to a Node<T>
            while current_node.is_some() {
                match &current_node.as_ref() {
                    Some(current) => {
                        print!("node({:?}) " , current.data);
                        current_node = &current.next;
                    }
                    None => { break; }
                }
            }

            

        
        }else{
            // list is empty 
            println!("head-> None");
        }
    }


    //pub fn insert_end(&mut self, element: T) {
    //    // This function inserts a new element in the end of the list
    //    let mut newnode = Node::new(element);  // function new returns a value of type Node
    //    if self.head.is_none() {
    //        self.head = Some(Box::new(newnode));
    //    }
    //    else{
    //        // H -> n1;next-> n2;next-> null 
    //        let mut lastnode = self.get_last();
    //        lastnode.next = newnode;
    //    }
    //}    
    //#[derive(Debug)]
    //fn get_last(&self) -> Option<Box<Node<T>>>{

        //match self.head {
        //    None => None, 
        //    Some(current_head) => {
        //        println!("{:?}", current_head);
        //    }
        //}
    //}


    //pub fn get_last(&self) -> Option<Box<Node<T>>>{
    //    
    //    let mut last = None;
    //    if self.head.is_some(){
    //        let mut current = &self.head;  // self.head ->  Option<Box<Node<T>>>
    //        //print!("HEAD-> node ({:?}), ",  current.as_ref().unwrap().data);   
    //        while current.is_some() {
    //            //if current.as_ref().unwrap().next.is_some(){ // I use this condition to avoid the case where the next is null 
    //            //    print!("node({:?}) next-> node({:?}), ", current.as_ref().unwrap().data, &current.as_ref().unwrap().next.as_ref().unwrap().data);
    //            //}else{  
    //            //    println!("node({:?}) next-> {:?}", current.as_ref().unwrap().data, &current.as_ref().unwrap().next);
    //            //}
    //            last = current.as_ref();
    //            current = &(current.as_ref().unwrap().next);
    //            // as_ref(&self) -> Option<&T> : converts from &Option<T> to Option<&T>
    //            // unwrap(self)  -> T : returns the contained Some value, consuming the self value.
    //        }
    //    }
    //    last 
    //}



}
#[allow(dead_code)]
pub fn print_type_of<T>(_: &T) {
    println!("TYPE: [{}]", std::any::type_name::<T>())
}