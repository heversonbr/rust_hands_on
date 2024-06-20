// TYPES of STRINGS:
// - str: also called string slice, it is the most primitive string type. 
//      Remember: a slice is a contiguous sequence of elements in a collection (https://doc.rust-lang.org/stable/book/ch04-03-slices.html)
//      It is an immutable, fixed lenght string, somewhere in memory.
//      It is usually seen in its borrowed form:  &str
//      It is also the type of string literals:  &'static str.  (https://en.wikipedia.org/wiki/String_literal)
//      String literals have a static lifetime, which means the string is guaranteed to be valid for the duration of the entire program.
//
// - String: A UTF-8–encoded (https://en.wikipedia.org/wiki/UTF-8), growable string  
//      It is a heap allocated data-structure, use when you need to modify or own string data. 
//      The String type is the most common string type that HAS OWNERSHIP over the contents of the string.
//
//      
//


#[allow(dead_code)]

pub fn run(){

    // str type : fixed size
    let hello: &str = "hello ";
    // -> String literals are string slices, which means we can explicitly specify hello’s lifetime as well, such this: 
    // let hello: &'static str = "Hello, world";
    println!("Length: {} ", hello.len());
    println!("{}", hello); 
    

    // String type: growable
    // CREATING STRINGS
    //      we can create a String from a literal string with 'String::from()'
    let mut world: String = String::from("world");
    //      but also in other multiple ways:
    //           String::from("any string literal");
    //           "any string literal".to_string();
    //           "any string literal".into();
    
    println!("Length: {}", world.len());
    println!("{}", world);


    // You can append a char to a String usin 'push'
    world.push('!');
    println!("{}", world);
    // we can append a slice string (&str) to a String using 'push_str'
    world.push_str(" Welcome to Rust!");
    println!("{}", world);


    // CAPACITY in bytes
    println!("{}", world.capacity());
    // is EMPTY
    println!("{}", world.is_empty());
    // CONTAINS
    println!("Contains world hello: {}", world.contains("hello"));
    println!("Contains world Rust: {}", world.contains("Rust"));

    // REPLACE
    println!("{}", world.replace("Rust", "Java"));
    //LOOP  through string by white space 
    for word in world.split_whitespace(){
        println!("split : {}", word);
    }
    // create string with capacity
    let mut s = String::with_capacity(20);
    s.push('a');
    s.push('b');
    println!("s: {}", s);
    
    assert_eq!(2, s.len());  // this prints nothing because the assertion is true
    //assert_eq!(3, s.len());  // this prints 'assertion failed' because the assertion is false
    assert_eq!(20, s.capacity());  // this prints nothing because the assertion of capacity is correct

}