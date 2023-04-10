// Raw pointer example
// Raw pointers * and references &T function similarly, but references are always 
// safe because they are guaranteed to point to valid data due to the borrow checker. 
// Dereferencing a raw pointer can only be done through an unsafe block.

// Raw pointers are often used in low-level code, such as when interfacing with C libraries 
// or working with memory-mapped hardware. However, it's generally recommended to use Rust's 
// safe abstractions, such as references and smart pointers, 
// whenever possible to ensure memory safety.

#[allow(dead_code)]
pub fn run(){

let x = 42;
println!("The original value stored in x is {:?}", &x);
//Raw pointers can be created using the & operator or the as keyword.
let raw_ptr: *const i32 = &x as *const i32;   // The as keyword is used to convert a reference to a raw pointer. 

println!("The value stored in raw_ptr is {:?}", &raw_ptr);
let y = unsafe { *raw_ptr };   // dereference of raw pointer is unsafe and requires unsafe function or block
// dereferencing a raw pointer can cause undefined behavior if the pointer is null or points to invalid memory. 
// The unsafe keyword signals to the compiler that we're taking responsibility for ensuring that the pointer is valid.

println!("The dereferenced value stored in y is {:?}", y);

}