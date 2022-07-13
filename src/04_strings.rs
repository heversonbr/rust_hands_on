// TYPES of STRINGS:
// - str (primitive string): Immutable fixed lenght string, somewhere in memory
// - String (growable string): heap allocated data-structure, use when you need to modify or own string data. 

pub fn run(){

    // str type
     let hello = "hello ";
    println!("Length: {} ", hello.len());
    println!("{}", hello);

    // String type
    let mut world = String::from("world");
    println!("Length: {}", world.len());
    println!("{}", world);
    // some methods
    // push single char
    world.push('!');
    println!("{}", world);
    // push string of chars
    world.push_str(" Welcome to Rust!");
    println!("{}", world);


    // Capacity in bytes
    println!("{}", world.capacity());
    // is empty
    println!("{}", world.is_empty());
    // contains
    println!("Contains world hello: {}", world.contains("hello"));
    println!("Contains world Rust: {}", world.contains("Rust"));
    // Replace
    println!("{}", world.replace("Rust", "Java"));
    //loop  through string by white space 
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