// This example tries to show why we need the Interior Mutability Pattern in Rust 

// Remind: 
// The borrow checker in Rust is a component of the Rust compiler that ensures memory safety 
// by enforcing strict rules on how references (or "borrows") to data are used. It helps prevent 
// common programming errors such as dangling pointers, data races, and use-after-free errors
// by ensuring that references to data are used in a way that adheres to Rust's ownership model.
// However, the borrow checker is inherently conservative: 
// We can insure that the borrow checker rejects 'memory unsafe programs' at compile time, 
// but, due to some limits of the static code analysis, it is impossible to the the borrow checker to 
// accept 'all memory safe programs'. 
// In other words, the set of memory safe programs accepted by the borrow checher is a subset of ALL memory safe programs.

// Therefore, sometimes we need to find a way around (circumvent) the borrow checker in order to achieve some goals.
// A consequence of the borrowing rules is that when you have an immutable value, you can’t borrow it mutably.
// However, there are situations in which it would be useful for a value to mutate itself in its methods 
// but appear immutable to other code. Code outside the value’s methods would not be able to mutate the value. 

// Let's see some examples that present this situation and how we can fix it with interior mutability. 

// Suppose we have a graph with connnected nodes, A, B, and C.  
// Nodes B and C point to node A, and each of them carry an integer value in it.
// Graphs by nature require that we have potentially many refereces to the same node. 
// We know that if these references are read-only references we are not going to have 
// any issue with it, because the borrow checker allow us to have as many immutable references as we need (see borrow checker rules).
// However, if we need to change these references the borrow checker will not allow us to do that (in compiling time).

// Check each of the following examples. 
mod example1;
mod example2;
mod example3;
mod example4;
mod example5;

fn main() {

    //example1::run();
    //example2::run();
    //example3::run();
   // example4::run();
    example5::run();

}
