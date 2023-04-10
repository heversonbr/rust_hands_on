// Smart Pointers.
// A pointer is a general concept for a variable that contains an address in memory. 
// This address refers to (or “points at”) some other data. 
// The most common kind of pointer in Rust is a reference

// Smart pointers, on the other hand, are data structures that act like a pointer
// but also have additional metadata and capabilities.
// Rust has a variety of smart pointers defined in the standard library that provide
// functionality beyond that provided by references.

// Rust, with its concept of ownership and borrowing, has an additional difference 
// between references and smart pointers: 
// - While references only borrow data, in many cases, smart pointers own the data they point to.
// - Unlike an ordinary struct, smart pointers implement the 'Deref' and 'Drop' traits. 

// The most common smart pointers in the standard library are:
//      - Box<T> for allocating values on the heap. 
//      - Rc<T>, a reference counting type that enables 'multiple ownership'
//      - Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing 
// rules at runtime instead of compile time.

// Boxes don’t have performance overhead, other than storing their data on the heap 
// instead of on the stack. You’ll use Box Smart pointers most often in these situations:
// - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
// - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
// - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

#[allow(dead_code)]
pub fn run(){
    // ------------------------------------------------------------------
    // This is a basic example that shows how to use a box to store a value on the heap:
    println!("\nCreating Box<i32> smart pointer: Box::new(1);");
    let boxed_int: Box<i32> = Box::new(1);
    // in this case, we can access the data in the box similar to how we would if this data were on the stack.
    println!("The value of box is: {:?} \n", boxed_int);
    // ------------------------------------------------------------------
    println!("\nCreating Box<Vec<i32>> smart pointer: Box::new(vec![1, 2, 3]);");
    let boxed_vec: Box<Vec<i32>> = Box::new(vec![1, 2, 3]); 
    println!("boxed_vec[0] : {:?}", boxed_vec.get(0)); 
    println!("boxed_vec[1] : {:?}", boxed_vec.get(1));
    println!("boxed_vec[2] : {:?}", boxed_vec.get(2));
    println!("boxed_vec[10] (out of bounds): {:?}", boxed_vec.get(10));
    // get => returns a reference to an element or subslice depending on the type of index.
    // If given a position, returns a reference to the element at that position or None if out of bounds.
    // If given a range, returns the subslice corresponding to that range, or None if out of bounds.
    // ------------------------------------------------------------------
    // Putting a single value on the heap isn’t very useful, 
    // so you won’t use boxes in this way very often.
    // In the next example in the file: 'smart_pointers_box.rs' we will look at the case 
    // where boxes allow us to define types that we wouldn’t be allowed to if we didn’t have boxes.
    // - Recursive Types with Boxes:
    // A value of recursive type can have another value of the same type as part of itself.
    // Recursive types pose an issue because at compile time Rust needs to know how much 
    // space a type takes up. However, the nesting of values of recursive types could 
    // theoretically continue infinitely, so Rust can’t know how much space the value needs.
    // Because boxes have a known size, we can enable recursive types by inserting 
    // a box in the recursive type definition.

    
}