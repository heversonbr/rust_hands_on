// TYPES of STRINGS:
// - str (primitive string): Immutable fixed lenght string, somewhere in memory
// - String (A UTF-8–encoded, growable string): heap allocated data-structure, use when you need to modify or own string data. 

#[allow(dead_code)]

pub fn run(){

    // str type(primitive type)
    let hello = "hello ";
    println!("Length: {} ", hello.len());
    println!("{}", hello);

    // String (growable) type
    let mut world: String = String::from("world");
    // The String can be created in multiple ways:
    //     String::from("any string literal");
    //     "any string literal".to_string();
    //     "any string literal".into();
    
    println!("Length: {}", world.len());
    println!("{}", world);

    // SOME METHODS:
    // push single char
    world.push('!');
    println!("{}", world);
    // push string of chars
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