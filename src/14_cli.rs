// lets check how to grab arguments passed to programs through the command line 


pub fn run(){

    let args: Vec<String> = std::env::args().collect();  
    // note that we can use 'use std::env in the preambule of this file and then call only 'env::args().collect();' 

    // Ex1: printing the vector of arguments.
    println!("Args: {:?}", args);
    // the first argument in the vector is the target of the executable
    // running this with 'cargo run'  must print 'Args: ["target/debug/ex1_print"]'
    // if we run "cargo run hello there" will print 'Args: ["target/debug/ex1_print", "hello", "there"]'

    // Ex2: getting the number of arguments using the lenght of the vector
    println!("Args: {:?}", args.len());
    if args.len() > 2 {
        println!("Wrong number of arguments, expecting max one arg!");
    }

    // Ex3: checking for specific arguments, comparing the received arguments to previously defined strings
    let command = args[1].clone();
    if command == "status" {
        println!("command {} received" , command);   
    }else{
        println!("received invalid command");
    }




}