// In this fifth example, we will use Interior Mutability in order to be able to change the 
// value of the Nodes. 
// There are 4 Smart pointers that implement Interior Mutability in the standard library: 
// Cell, RefCell, RwLock and Mutex
// In this example we are using Cell, and we mention the reason later.
use std::cell::Cell;

#[allow(dead_code)]
#[derive(Debug)]
struct Node<'a>{
    // So lets wrap the value of each node in a Cell 
    value: Cell<i32>,
    neighbours: Vec<&'a Node<'a>>    

}

// Cell allows us to mutate the value of the contained value even if we only have a shared reference 
// of that Cell. 
// Cell allows either mutate the contained value or to get a copy of the contained value. 
// here we will get a copy of the value by using the get and set methods provided by Cell 
fn add_one(node: &Node){
   let current_value = node.value.get();
    node.value.set(current_value + 1);

   for neighbour in node.neighbours.iter() {
       add_one(&neighbour);
   }
} 


#[allow(dead_code)]
pub fn run(){ 

    let a = Node {
        // we also need to change the way we instantiate the value, now using Cell wrapping our integer
        value: Cell::new(1),
        neighbours: vec![],
    };

    let b = Node {
        value: Cell::new(2),
        neighbours: vec![&a],
    };

    let c = Node {
        value: Cell::new(3),
        neighbours: vec![&a],
    };
   
    println!("Immutable struct Node a:");
    println!("{:?}", &a);
    println!("Immutable struct Node b:");
    println!("{:?}", &b);
    println!("Immutable struct Node c:");
    println!("{:?}", &c);
     
    println!("Add 1 to node b:");
    add_one(&b);
    

    println!("Immutable struct Node a:");
    println!("{:?}", &a);
    println!("Immutable struct Node b:");
    println!("{:?}", &b);
    println!("Immutable struct Node c:");
    println!("{:?}", &c);

    println!("Add 1 to node c:");
    add_one(&c);

    println!("Immutable struct Node a:");
    println!("{:?}", &a);
    println!("Immutable struct Node b:");
    println!("{:?}", &b);
    println!("Immutable struct Node c:");
    println!("{:?}", &c);

    // now we can see that we can change the value of the nodes even with shared references. 
    // by using interior mutability.
    // So, if we have a piece of data that is 'copiable' and copying is not going to have 
    // an important impact on the performance we can just use Cell. 
    // the next example we will discuss the case where we have a data that is not 'copiable'
    // like a string. 
    // see the next example6 for that. TODO  
   
    
}


 