// Option, which is another enum defined by the standard library. 
// The Option type encodes the very common scenario in which a value could be 'something' or it could be 'nothing'.
// Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the cases 
// you should be handling; this functionality can prevent bugs that are extremely common in other programming languages.
// ust doesn’t have the null feature that many other languages have. Null is a value that means there is no value there. 
// In languages with null, variables can always be in one of two states: null or not-null.
// However, the concept that null is trying to express is still a useful one: a null is a value that is currently 
// invalid or absent for some reason. The problem isn’t really with the concept but with the particular implementation. 
// As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present 
// or absent. This enum is Option<T>:
//     enum Option<T> {
//         None,
//         Some(T),
//     }
// Ref: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html?highlight=option#the-option-enum-and-its-advantages-over-null-values
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run(){

    let some_number = Some(5);
    let some_char = Some('e');
    // Rust can infer the previous types because we’ve specified a value inside the Some variant. 
    // in the case of the 'None', the compiler can’t infer the type that the corresponding Some variant 
    // will hold by looking only at a 'None'
    let absent_number: Option<i32> = None;
    
    // NOTE: 
    // You have to convert an Option<T> to a T before you can perform T operations with it.
    // Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.
    // In general, in order to use an Option<T> value, you want to have code that will handle each variant. 
    // You want some code that will run only when you have a Some(T) value, and this code is allowed to use the inner T. 
    // You want some other code to run if you have a None value, and that code doesn’t have a T value available. 
    // The match expression is a control flow construct that does just this when used with enums: 
    // it will run different code depending on which variant of the enum it has

    match some_number {
        Some(inner_value) => println!("some_number is {:?}", inner_value),
        None => println!("some_number: None value found"),
    }
    match some_char {
        Some(inner_value) => println!("some_char is {:?}", inner_value),
        None => println!("some_char: None value found"),
    }
    match absent_number {
        Some(absent_value) => println!("absent_value is {:?}", absent_value),
        None => println!("absent_number: is Option::None"),

    }


    
    
}