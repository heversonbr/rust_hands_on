// Smart Pointers: Box<T> : 

// Box is a type of smart pointer that allows you to allocate memory and store a value on the heap.
// The Box itself is stored on the stack and has a fixed size, regardless of the size of the value it points to.
// When the Box goes out of scope, Rust automatically deallocates the memory it points to, 
// which in turn drops the value stored in that memory.

// This is different from the stack where variables are automatically deallocated when they go out of scope. 
// With Box, the memory is freed when the Box goes out of scope, 
// but you can also explicitly call the drop method to free the memory.
// When you create a Box, you become the owner of the memory it points to. 
// You can transfer ownership of the Box and its contents by moving it to another variable or passing it as an argument to a function. 
// But you cannot have multiple owners of the same Box at the same time.

// why would you want to store data on the heap instead of the stack? 
// There are a few reasons for this. 
// First, the heap is generally much larger than the stack, 
// so it can accommodate larger data structures. 
// For example, let’s say we have a vector (a growable array) that contains a million 
// elements. If we try to store this vector on the stack, we would likely run out of 
// stack space and get a runtime error. However, if we were to store the vector in a box,
// we can allocate it on the heap and avoid this problem.
// Secondly, data stored on the heap can be shared between different parts of a program,
// whereas data on the stack is local to the function in which it is declared making 
// it accessible in the function itself and sharing data actually means copying data 
// around or passing references. This can be especially useful when we want to share 
// data between threads, or when we want to create recursive data structures 
// (e.g., a linked list or a tree) that reference themselves.

#[allow(dead_code)]
pub fn run(){
    // ------------------------------------------------------------------
    // storing a string in the heap
    println!("1) Using Box to store a string on the heap:");
    let s = Box::new("hello");
    println!("s: {:?}", s);
    println!("s: {:?}", *s);    
    // we can also use the method deref :
    println!("s: {:?}", std::ops::Deref::deref(&s));
    
    // ------------------------------------------------------------------
    // In this example, the List enum has two variants: 
    // Cons, which represents a node in the list with a value and a pointer
    // to the next node, and Nil, which represents the end of the list.
    // The Cons variant contains a value of type T and a Box<List<T>> 
    // pointing to the next node in the list.
    println!("2) Using Box for Recursive data structures:");
    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>),
       Nil,
    }
    let first = Box::new(List::Nil); 
    let second = Box::new(List::Cons(2,first)); 
    let third = List::Cons(1, second);

    let my_list: List<i32> =  third; 
    println!("my_list => {:?}", my_list);

    let another_list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("another_list => {:?}", another_list);

    let root: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Cons(4, Box::new(List::Nil)))))))); 
    println!("root => {:?}", root);

    // Because the size of the List data structure can be variable depending on the 
    // number of nodes it contains, it is necessary to use Box to store the data on the 
    // heap rather than including it directly in the structure. 
    // This allows the data structure to be allocated and deallocated dynamically at runtime.
    // ------------------------------------------------------------------
    // this example panics due to the fact that we will try to use a value 
    // after moving it. 
    //{ 
    //    let a = Box::new(5);
    //    let b = a;              // value moved here, 
    //    println!("a = {}", a);  // error: use of moved value:=> 'value borrowed here after move'
    //    println!("b = {}", b);
    //}
    // ------------------------------------------------------------------
    // another example: now using a node
    println!{"--------------------------"};
    // use uuid::Version::Nil;

    #[derive(Debug)]
    struct Node {
        elem: i32,
        next: Option<Box<Node>>
    }
    
    let mut root: Option<Box<Node>>  = None;    
    println!{"Root=> {:?}", &root};
    
    //root = Some(Box::new(node));
    let mut node = Node { elem : 10, next : None };
    println!{"ADDING NODE1"};
    println!{"{:?}", &node};
    
    if root.is_none() {
        println!("root None, add first");
        root = Some(Box::new(node));
    }else{
        node.next = root.take();
        //println!{"node: {:?}", &node };
        root = Some(Box::new(node));

    }
    
    println!{"Root=> {:?}", &root};
    
    let mut  node = Node { elem : 20, next : None };
    println!{"ADDING NODE2"};
    println!{"{:?}", &node};

    if root.is_none() {
        println!("root None, add first");
        root = Some(Box::new(node));
    }else{
        node.next = root.take();
        //println!{"node: {:?}", &node };
        root = Some(Box::new(node));
    }

    

    println!{"Root=> {:?}", &root};
    let mut node = Node { elem : 30, next : None };
    println!{"{:?}", &node};

    if root.is_none() {
        println!("root None, add first");
        root = Some(Box::new(node));
    }else{
        node.next = root.take();
        //println!{"node: {:?}", &node };
        root = Some(Box::new(node));
    }

    println!{"Root=> {:?}", root};
   








    //struct Node<T> {
    //    data: T,
    //    next: Option<Box<Node<T>>>,
    //}
//
    //impl<T> Node<T> {
    //    /// Function 'new' receives a type T and returns a new structure of 
    //    /// type Node (with data type T)
    //    fn new(data: T) -> Self  {
    //        Node {
    //            data : data,
    //            next : None  // initially references a None, which will be set later
    //        }
    //    }
    //}
//
    //let my_node = Node::new(value)
    
 
    
}