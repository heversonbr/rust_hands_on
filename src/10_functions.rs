// Functions used to store blocks of code for reusing
// Functions are declared using the fn keyword. Its arguments are 
//           type annotated, just like variables, and, if the function returns a value, 
//           the return type must be specified after an arrow ->

pub fn run(){

    greeting("Hello", "John");
    
    // passing arguments and getting the return back
    println!("result: {}", add(1,2));

    println!("sum result without biding function: {}", add(5,15));

    // bind function values to variables
    let get_sum = add(5,5);
    println!("sum result using biding function to variable: {}", get_sum);

    // Closures: closures are anonymous functions.
    // you can save in a variable or pass as arguments to other functions. 
    // You can create the closure in one place and then call the closure to evaluate it in a different context. 
    // Unlike functions, closures can capture values from the scope in which they’re defined. 
    // By themselves, closures aren’t all that interesting, but when you combine them with functions that take closures as arguments, really powerful things are possible.
    // We create a closure using the |...| { ... } syntax, and then we create a binding so we can use it later.
    // More about closures here:  https://medium.com/coding-rust/best-explanation-of-closure-in-rust-2b20210eba53
    // Ex: 
    let add_numbs = | n1: i32 , n2: i32 | { n1 + n2 };
    println!("C sum with closures : {} ", add_numbs(4,3));

}

// passing parameters 
fn greeting(greet: &str, name: &str){

    println!("{} {} , nice to meet you! ", greet, name)

}

// returning values 
// we use the arrow to indicate the returning type of  a function
fn add(mut number1: i32, mut number2: i32) -> i32 {
    println!("Received number1: {} , number2: {}!", number1, number2);
    number1 +=2;
    number2 +=3; 
    println!("Added , number1: {} , number2: {}!", number1, number2);

    // to return a value just remove the semicolon to tell the function to return it
    // this will return 'number1 + number2' 
    number1 + number2
}




