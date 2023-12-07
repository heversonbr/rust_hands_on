// What Is Ownership?
// Ownership is a set of rules that governs how a Rust program manages memory. 
// All programs have to manage the way they use a computer’s memory while running. 
// Some languages have garbage collection that regularly looks for no-longer used memory 
// as the program runs; in other languages, the programmer must explicitly allocate and free the memory.
// Rust uses a third approach: memory is managed through a system of ownership with a set of rules that 
// the compiler checks. If any of the rules are violated, the program won’t compile. 
// Ref: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// In order to understand ownership it is important to have in mind how the stack and heap memory works. 
// Both the stack and the heap are parts of memory available to your code to use at runtime, 
// but they are structured in different ways. The stack stores values in the order it gets them and 
// removes the values in the opposite order. This is referred to as last in, first out.
// All data stored on the stack must have a known, fixed size. 
// Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
// The heap is less organized: when you put data on the heap, you request a certain amount of space. 
// The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, 
// and returns a pointer, which is the address of that location. This process is called allocating on the heap 
// and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). 
// Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, 
// but when you want the actual data, you must follow the pointer.
// Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, 
// and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.

// Ownership Rules
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

// In order to make it easy the concept of ownership lets use strings as example.
// We have two types of strings in RUST: str and String. 
// - str (primitive string or string literals): are immutable fixed lenght strings, somewhere in memory (stack)
// - String (growable strings): store an amount of text that is unknown to us at compile time. It is a heap allocated data-structure, used when you need to modify string data. 
#[allow(dead_code)]
pub fn run(){
// - 1) As a first example of ownership, we’ll look at the scope of some variables. 
// A scope is the range within a program for which an item is valid. 
// The variable is valid from the point at which it’s declared until the end of the current scope.

// -- Ex1: string literals (str):  In this type, the string value is hardcoded into our program at compile time. 
    println!("-- Example 1: ");
    {
        let a = "Hello World";
        println!("Variable a is a string literal: {}" , a);
    }
    // uncomment the following line to see that an error ('not found in this scope') will appear because the variable a becomes invalid out of its scope
    // println!("Variable a is a string literal: {}" , a); 

// -- Ex2: String Type: This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
    println!("-- Example 2: ");
    {
        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str() appends a literal to a String
        println!("String type : {}", s); // This will print `hello, world!`
    }
        
// Why can String be mutated but literals cannot? The difference is how these two types deal with memory.
// In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable.
// This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. 
// With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, 
// unknown at compile time, to hold the contents. 
// To achieve that we need: 
// a) The memory must be requested from the memory allocator at runtime.
// b) We need a way of returning this memory to the allocator when we’re done with our String. 
// the first item is achieved as in any other language, for instance when we call a function the implementation requests the memory it needs.
// the second item is usually achieved by manually checking if the variable in the memory in not need anymore and free it. or using gabage collectors.
// rust uses a different approach: the memory is automatically returned once the variable that owns it goes out of scope.


// - Ex3:  When a heap-allocated variable leaves its scope, rust calls the special function 'Drop'. 
//         Rust calls drop automatically at the closing curly bracket.
    println!("-- Example 3: ");
    {  // start scope
        let mut g = String::from("hello");      // g is valid from this point forward
        g.push_str(", world!");                 // do stuffs with g
        println!("String type in a scope : {}", g);        
    }
    // this scope is now over, and g is no longer valid, uncomment the line to check.
    //println!("String type : {}", g); 

    //  This pattern has a profound impact on the way Rust code is written.
    //  It may seem simple right now, but the behavior of code can be unexpected in more complicated situations when we want 
    //  to have multiple variables use the data we’ve allocated on the heap.


// - Ex4: Interactions of Variables and Data : MOVE
//  a) fixed-size simple values such as integers:  5 is assinged to x and a copy of x is assigned to y.
//     This happens because the values have fixed size and are therefore the value 5 is pushed twice on the stack. 
    println!("-- Example 4.a: ");
    {
        let x = 5;
        let y = x;
        // in this case, as the value is fixed-size and allocated on the stack, when we assign y with the value of x, 
        // a new copy of the value is created and each copy has a different owner. 

        println!(" x : {} , y: {} ", x, y);
    }

//  b)  growable strings: A String is made up of three parts: 
//                         a pointer to the memory that holds the contents of the string, a length, and a capacity. 
//      This group of data is stored on the stack and the content itself is stored on the heap.
    println!("-- Example 4.b: ");
    {
        let s1 = String::from("hello");
        let s2 = s1;    // values on the stack are moved to s2. s1 will be invalidated
        // In this case when we assign s1 to s2, the String data on the stack is copied, that is we creat a copy of the pointer, length, and capacity. 
        // We do NOT create a copy of the data on the heap, because it would be very expensive in terms of runtime performance.
        // To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid. 
        // Therefore, Rust doesn’t need to free anything when s1 goes out of scope.
        // if you uncomment next line you will see an error because, s1 is invalidated and Rust compiler prevents you from using the invalidated reference. 
        //println!("{}, world!", s1);
        println!("s2 (moved from s1): {}, world!", s2);
    }
// - Ex5: Interacttions of Variables and Data : CLONE
//        If we do want/need to copy the heap data of the String, not just the stack data, we can use a common method called CLONE.
    println!("-- Example 5: ");
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("S2 (cloned from S1) -> s1 = {}, s2 = {}", s1, s2);
    }

// - Ex6: The case of Stack-Only Data. When data is stored only on the stack we will COPY (deep copy) the full data
    println!("-- Example 6: ");
    {
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);
        // as we fully copy the data on the stack , the origin keeps its validity after the assignment and each variable owns a copy.
    }


}

// TAKEAWAY:
//
// MOVE:  assigning one variable to another transfers the ownership to the assignee.   
//        When the variable v is moved to v1, the object on the stack is copied. 
//        What has essentially happened in the previous example is a shallow copy (only the references are copied, the data on the heap stays intact)
//        Assignment is not the only operation which involves moves. 
//        Values are also MOVED when 'passed as arguments' or 'returned from functions', or assigned to members of a struct or enum.
//        This change of ownership is good because if access was allowed through both v and v1 then you will end up with two stack objects pointing to the same heap buffer.
//
// COPY: When values are contained entirely in the stack. There is nothing to own on the heap. That is why it is ok to allow access through both independent copies. 
//       All primitive types like integers, floats and characters are Copy. Structs or enums are not Copy by default but you can derive the Copy trait. 
//       In general, any type that implements Drop cannot be Copy because Drop is implemented by types which own some resource and hence cannot be simply bitwise copied. 
//       But Copy types should be trivially copyable. Hence, Drop and Copy don't mix well. 
//
// CLONE: for values on the heap, if you want to create a deep copy (copy both stack and heap values), the code should call the clone method.
//        Due to deep copying, both copies are free to independently drop their heap buffers.
//
// The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.
// When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless 
// ownership of the data has been moved to another variable.
//
// Types that implement COPY (check the type documentation to get full information):
// - All the integer types, such as u32.
// - The Boolean type, bool, with values true and false.
// - All the floating point types, such as f64.
// - The character type, char.
// - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
// 

// Ref: https://hashrust.com/blog/moves-copies-and-clones-in-rust/
