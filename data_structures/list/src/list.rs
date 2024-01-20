// Notes: 
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
pub fn print_type_of<T>(_: &T) {
    println!("TYPE: [{}]", std::any::type_name::<T>())
}

#[allow(dead_code)]
#[derive(PartialEq)]
#[derive(Debug)]
// Node struct to be added on the list
pub struct Node<T> {   
    data: T,
    next: Option<Box<Node<T>>>
} 

#[allow(dead_code)]
#[derive(Debug)]
// Single Linked List struct 
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
    fn new(data: T) -> Self  {
        Node {
            data : data,
            next : None  // initially references a None, which will be set later
        }
    }
    
    pub fn set_next(&mut self, next_node: Node<T>){
        self.next = Some(Box::new(next_node));
    }

    pub fn get_next(&self) -> &Option<Box<Node<T>>>{
        &self.next
    }

    pub fn set_data(&mut self, data: T) { 
        self.data = data;
    }

    pub fn get_data(&self) -> &T { 
        &self.data
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

    /// Function 'len' returns the current lenght of the list
    pub fn len(&self) -> usize { 
        self.lenght
    }

    pub fn get_head(&self) -> &Option<Box<Node<T>>> {
        &self.head
    }
    // TODO : check 'using box as references' on the book! 

    // In a list a node can be added in three ways: 
    //   1) at the front of the linked list  
    //   2) after a given node. 
    //   3) at the end of the linked list.

    /// Function 'push_front' adds a new Node of type T in the front of the list
    /// Arguments: receives the value of type T corresponding to the value of the new node
    pub fn push_front(&mut self, data: T) {

        print!("Inserting value [{:?}] at the beginning of the list\n", data);

        let current_head = self.head.take(); 
        let mut newnode = Node::new(data);    // New node with data equals to given value
       
        match current_head {
            None => {
                self.head = Some(Box::new(newnode));
            }, 
            Some(node) => { 
                newnode.next = Some(node);
                self.head = Some(Box::new(newnode));
             }
        }

        // Using If and take:
        //if self.head.is_none() {           
        //   self.head = Some(Box::new(newnode));  
        //}else{
        // approach: Modifying an Option in-place with take: https://doc.rust-lang.org/stable/std/option/index.html#modifying-an-option-in-place
        // take() : takes ownership of the contained value of an Option, if any, replacing the Option with None
        //    newnode.next = self.head.take();    
        //    self.head = Some(Box::new(newnode));
        //}

        // increase the length of the list
        self.lenght+=1;  

    }

    pub fn show(&self){
        println!("Print list:");
        if self.head.is_some(){  // list in NOT empty:  head has an Option that is a ref to a Node: &Node
            let mut current_node = &self.head;  // self.head` has type `Option<Box<Node<T>>> , we pass a ref to head to avoid moving the ownership 
                                                //  then current might be a &Option that is a ref (Box) to a Node<T>
            while current_node.is_some() {
                match &current_node.as_ref() {  // as_ref(): Converts this type into a shared reference of the (usually inferred) input type.
                    Some(current) => {          // as_ref():  If you don’t want to take ownership of the value stored in the Box, you can use the as_ref() method to get an immutable reference to the value.
                        print!("node({:?}) " , current.data);
                        current_node = &current.next;
                    }
                    None => { break; }
                }
            }
            println!("");
        }else{
            // list is empty 
            println!("head-> None");
        }
    }

    /// Function 'push_end' adds a new Node of type T in the end of the list
    /// Arguments: receives the value of type T corresponding to the value of the new node
    pub fn push_end(&mut self, data: T) {
        //TODO: test it here
        // This function inserts a new element in the end of the list
        let newnode = Node::new(data);  // function new returns a new Node with value of type T

        if self.head.is_none() {
            // if head is none, list is empty, just create the first node. 
            self.head = Some(Box::new(newnode));
        }
       else{
            // Head not null: find last node and add new node after it. 
            // Ex: head -> (n1-next) -> (n2-next)-> (n3-next) -> null 
            // self.get_last() returns  Option<&mut Node<T>> 
            
            if let Some(_lastnode) = self.get_last() {
                //lastnode.set_next(newnode)
            }
        }
        self.lenght+=1; 
    } 

    /// Function 'get_mut_last' returns a mutable reference to the last node of the list
    // approach: traverses the list from head to last node, and return a mutable reference to it. 
    pub fn get_last(&mut self) -> Option<&mut Node<T>> {
    
        if self.head.is_some(){   // self.head -> Option<Box<Node<T>>>
            let mut current_node = self.head.as_mut(); 
             // as_mut(): Converts this type into a mutable reference of the (usually inferred) input type.

            while current_node.is_some() {              
                // as_mut(): If you need to mutate the value stored in the Box, 
                // you can use the as_mut() method to get a mutable reference to the value.
                match current_node{
                    Some(current) => { 
                        //println!("current node => ({:?}) " , &current.data);
                        if current.next.is_none(){
                           return Some(current);
                        }
                        current_node = current.next.as_mut();
                    }
                    None => { break ; }
                }
            }
        }
        None
    }


    pub fn get_node(&mut self, data: T) -> Option<& Node<T>> {
    // traverse the list to find a node that has in its data field the same value of 'data' passed as argument 
    // returns either a ref to the node, if exists, or none. 

        if self.head.is_some(){
            let mut current_node = self.head.as_mut();  
            // self.head ->  Option<Box<Node<T>>>
            while current_node.is_some() {    
                match current_node{
                    Some(current) => { 
                        //println!("current node => ({:?}) " , &current.data);
                        if current.data == data {
                           return Some(current);
                        }
                        current_node = current.next.as_mut();
                    }
                    None => { break ; }
                }
            }
        }
        None

    }


    pub fn pop_front(&mut self) -> Option<T>{
        
        let current_head = self.head.take(); 

        match current_head {
            Some(first_node) => {
                self.head = first_node.next;
                Some(first_node.data)
            },
            None => {
                None
            }
        }

    }

    //TODO 
    fn pop_back(){
        todo!(); 



    }

}

