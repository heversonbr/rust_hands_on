// Pointers are an important concept in Rust programming, 
// just as they are in many other programming languages. 
// In Rust, there are three main types of pointers: references, raw pointers and smart pointers.
//
// References, which are safe pointers that allow you to refer to a value WITHOUT TAKING OWNERSHIP of it.
// - References come in two flavors: immutable references (&T) and mutable references (&mut T).
// - Raw pointers: which are UNSAFE pointers that allow you to perform low-level operations 
//                 such as pointer arithmetic and dereferencing. Raw pointers are created 
//                 using the *const T or *mut T syntax.
// - Smart pointers: which are pointers that provide ADDITIONAL FUNCTIONALITY beyond 
//                   references and raw pointers. Examples of smart pointers in Rust 
//                   include Box, Rc, Arc, Cell, RefCell, Mutex, and RwLock. 
//                   Smart pointers are called "smart" because they perform additional 
//                   operations beyond simple pointer dereferencing, such as automatic 
//                   memory management, reference counting, interior mutability, and mutual exclusion.
// ------------------------------------------------
// Reference pointers point to a resource in memory 
#[allow(dead_code)]
pub fn run(){

    // Primitive array:
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("Array Values: {:?} ", (arr1, arr2));

    // With primitive data (ex: vectors), if you assign another variable to a place of data,
    // the first variable will no longer hold that value. 
    // You need to use a reference to point to the resource. 

    // Ex:  as vector is not a primitive value
    let vec1 = vec![1,2,3];
    // assigning the vector to another variable like this => let vec2 = vec1; 
    //  will catch an error, because we transfer the ownership. 
    // we should use the 'ampersand' symbol '&' in this case
    let vec2 = &vec1;
    println!("Vec Values: {:?} ", (&vec1, vec2));


}

