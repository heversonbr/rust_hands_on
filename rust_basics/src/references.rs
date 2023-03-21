// Remind: 
// A pointer is a general concept for a variable that contains an address in memory.    
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
// References (borrowing): 
// References are safe pointers that allow you to refer to a value without taking ownership of it. 
// They are created using the ampersand (&) symbol and can be either mutable or immutable.
// Immutable references allow you to read the value they point to but not modify it, 
// while mutable references allow you to both read and modify the value.
// References borrow the value they point to. They don’t have any special capabilities other than referring to data.

// Rust references are very simple at runtime: they are plain memory addresses. 
// At compile time, in contrast, references participate in more complex compiler analysis. 
// For example, references help to prove memory safety of a program.
// A reference is like a pointer in that it’s an address we can follow to access the 
// data stored at that address; that data is owned by some other variable.
// Unlike a raw pointer, a reference is guaranteed to point to a valid value of a particular 
// type for the life of that reference.
// These ampersands represent references, and they allow you to refer to some value without taking ownership of it.
// We call the action of creating a reference BORROWING.
// Likewise, the signature of a function can also use a '&' to indicate that the type of some of its parameters is a reference.
// Ref: https://hashrust.com/blog/references-in-rust/
//      https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

#[allow(dead_code)]
pub fn run(){
    // --------------------------
    // ex1: basics of references, 
    //      suppose we have a variable x that owns a value 42
    println!("Example 1: basic references (borrowing)");
    {
        let x: i32 = 42;
        //  in order to create a reference to x we use the 'ampersand' symbol '&'
        //  The '&x' syntax lets us create a reference that refers to the value of 'x' but does not own it. 
        //  Because it does not own it, the value (it points to) will not be dropped when the reference stops being used.     
        { 
            let r = &x; 
            //  to get the value corresponding to the address pointed by r, we use the operator '*'
            println!("r: {:?} ", *r); 
        }  
        println!("x: {:?} ", x);
        
    }
    // if we try to change the value it wont be possible because all variable and references above are immutable by default.
   
    // --------------------------
    // Ex2: Mutable References : change the value through a reference, 
    //      by using a mutable reference.
    //      In order to change the value using the reference, both the 
    //      variable owning the value and the reference must be mutable.
    println!("Example 2: change the value of a variable by using a mutable reference.");
    {
        let mut z: i32 = 52;
        let m = &mut z;
        println!("m: {:?} ", m);
        // to access the value corresponding to the address pointed 
        // by the mutable reference 'm' we use the operator '*'
        *m = 100;
        println!("changed m: {:?} ", m);
    }
    // This makes possible 'm' to mutate the value it borrows from 'z'.
    // --------------------------
    // Ex3: Shared and mutable references
    //      Mutable references have one big restriction: 
    //            if you have a mutable reference to a value, 
    //            you can have no other references to that value.
    //
    //      NOTE: a reference’s scope starts from where it is introduced and continues through the last time that reference is used. 
    println!("Example 3: Shared and mutable references");
    {
        let mut original_owner_x: i32 = 12;
        let mut_ref_1: &mut i32 = &mut original_owner_x;   // the scope of the reference 'mut_ref_1' starts here.
        //let mut_ref_2: &mut i32 = &mut original_owner_x; // this would generate an error, it wont be allowed until the scope of the first reference is active.
        println!("mut_ref_1 is {}", mut_ref_1);            // the scope of the reference 'mut_ref_1' ends here. the last time it is used.
        //println!("mut_ref_2 is {}", mut_ref_2);
        // note: this code wont compile: "error: cannot borrow `s` as mutable more than once at a time"
    }
    //      The usage rules for immutable and mutable references are as follows. 
    //      For a value, there can be:
    //      - EITHER MANY IMMUTABLE REFERENCES.
    //      - OR ONE MUTABLE REFERENCE.   
    //      But never both at the same time. 
    //      For this reason, another term for immutable references is SHARED REFERENCES.
    //      And a similar term for mutable references is EXCLUSIVE REFERENCES.
    // --------------------------
    // Ex4: DANGLING References: 
    // In languages with pointers, it’s easy to erroneously create a DANGLING pointer 
    // (i.e., a pointer that references a location in memory that may have been given to someone else) 
    // by freeing some memory while preserving a pointer to that memory. 
    // In Rust, by contrast, the compiler guarantees that references will never be dangling references: 
    // if you have a reference to some data, the compiler will ensure that the data will not go out of scope 
    // before the reference to the data does.
    //
    // the example below wont compile because the compiler finds out that there is a reference that points to no value.
    // uncomment to see the error:
    //  this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    // {
    //     fn mymain() {
    //         let reference_to_nothing = dangle();
    //     }
    //     
    //     fn dangle() -> &String {
    //         let s = String::from("hello");
    //         &s
    //     }
    //     mymain();
    // }

// TAKEAWAY:
// The Rules of References
//  - At any given time, you can have either one mutable reference or any number of immutable references.
//  - References must always be valid.



}