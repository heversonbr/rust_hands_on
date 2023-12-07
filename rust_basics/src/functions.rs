/// Functions used to store blocks of code for reusing
/// Functions are declared using the fn keyword. Its arguments are 
///           type annotated, just like variables, and, if the functoin returns a value, 
///           the return type must be specified after an arrow ->
/// 
/// Syntax and Structure of functions:
///      fn function_name(parameter1: type1, parameter2: type2) -> return_type {
///         // Function body
///         return value;
///      } 
#[allow(dead_code)]
pub fn run(){

    // Example 1: 
    // passing the values to the functions
    greeting("Hello", "John");
    
    // Example 2: 
    // passing arguments and getting the return back
    println!("result: {}", add(1,2));

    println!("sum result without biding function values: {}", add(5,15));

    // Example 3: 
    // bind function values to variables
    let get_sum = add(5,5);
    println!("sum result using biding function to variable: {}", get_sum);

    // Example 4: 
    // Closures: closures are anonymous functions.
    // you can save in a variable or pass as arguments to other functions. 
    // You can create the closure in one place and then call the closure to evaluate 
    // it in a different context. Unlike functions, closures can capture values from 
    // the scope in which they’re defined. By themselves, closures aren’t all that 
    // interesting, but when you combine them with functions that take closures as 
    // arguments, really powerful things are possible. We create a closure using the 
    // |...| { ... } syntax, and then we create a binding so we can use it later.
    // More about closures here:  
    // https://medium.com/coding-rust/best-explanation-of-closure-in-rust-2b20210eba53
    // Ex: 
    let add_numbs = | n1: i32 , n2: i32 | { n1 + n2 };
    println!("C sum with closures : {} ", add_numbs(4,3));


    // Example 5: generic functions
    let number_list = vec![34, 25, 11, 73, 5, 32, 13];
    let char_list = vec!['h', 'm', 'a', 'q'];

    println!("The largest number is {}", largest(&number_list));
    println!("The largest number is {}", largest(&char_list));



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

// GENERIC FUNCTIONS
// In Rust, Generic functions are very useful. Generic make code more flexible and provide more 
// functionality to the callers of the function. It prevents code duplication as it is not 
// required to define different functions of different types
// When defining a function that uses generics, we place the generics in the signature of the 
// function where we would usually specify the data types of the parameters and return value. 
// Doing so makes our code more flexible and provides more functionality to callers of our
// function while preventing code duplication.
// To parameterize the types in a new single function, we need to name the type parameter, 
// just as we do for the value parameters to a function. You can use any identifier as a type 
// parameter name. But we’ll use T because, by convention, parameter names in Rust are short, 
// often just a letter, and Rust’s type-naming convention is CamelCase. Short for “type,” T is 
// the default choice of most Rust programmers.
// Ref: https://doc.rust-lang.org/book/ch10-01-syntax.html 


fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // We use the T to define the generic type for the function and the types of parameters and return.
    // 'std::cmp::PartialOrd' is a trait that enables comparisons
    // we restrict the types valid for T to only those that implement PartialOrd 
    let mut largest  = &list[0];
    for item in list {
        if item > largest{
            largest = item;
        }
    }

    
    largest
}








