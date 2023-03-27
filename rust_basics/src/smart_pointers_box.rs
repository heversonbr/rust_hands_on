// Smart Pointers: Box<T> : 
// Box is a type of smart pointer that allows you to allocate memory on the heap and store a value in it. 
// The Box itself is stored on the stack and has a fixed size, regardless of the size of the value it points to.
// When the Box goes out of scope, Rust automatically deallocates the memory it points to, which in turn drops the value stored in that memory.


// This is different from the stack where variables are automatically deallocated when they go out of scope. 
// With Box, the memory is freed when the Box goes out of scope, but you can also explicitly call the drop method to free the memory.
// When you create a Box, you become the owner of the memory it points to. 
// You can transfer ownership of the Box and its contents by moving it to another variable or passing it as an argument to a function. 
// But you cannot have multiple owners of the same Box at the same time.







#[allow(dead_code)]
pub fn run(){
    // ------------------------------------------------------------------
    // how to use a box to store an i32 value on the heap:
    // in this case, we can access the data in the box similar to how we would if this data were on the stack.
    println!("\nCreating Box<i32> smart pointer: Box::new(1);");
    let boxed_int: Box<i32> = Box::new(1);
    println!("The value of box is: {:?} \n", *boxed_int);
    // note that the 'dereferencing' is often done implicitly for us by the compiler
    // ------------------------------------------------------------------
    println!("Creating Box<Vec<i32>> smart pointer: Box::new(vec![1, 2, 3]);");
    let boxed_vec: Box<Vec<i32>> = Box::new(vec![1, 2, 3]); 
    println!("boxed_vec : {:?}", *boxed_vec); 
    println!("boxed_vec[0] : {:?}", boxed_vec.get(0)); 
    println!("boxed_vec[1] : {:?}", boxed_vec.get(1));
    println!("boxed_vec[2] : {:?}", boxed_vec.get(2));
    println!("boxed_vec[10] (out of bounds): {:?}", boxed_vec.get(10));
    // get => returns a reference to an element or subslice depending on the type of index.
    // If given a position, returns a reference to the element at that position or None if out of bounds.
    // If given a range, returns the subslice corresponding to that range, or None if out of bounds.
    // ------------------------------------------------------------------
       
}