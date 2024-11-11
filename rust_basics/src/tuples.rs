
// Compound types can group multiple values into one type. 
// Rust has two primitive compound types: TUPLES and ARRAYS.
// ---------------------------------------------------------
// Tuples : group together values that can be of different types
//          max 12 values (elements)
// A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
// Tuples have a fixed length: once declared, they cannot grow or shrink in size.
// We create a tuple by writing a comma-separated list of values inside parentheses.
// Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.

#[allow(dead_code)]

pub fn run(){

    let person: (&str, &str, i8)  = ("John" , "Doe", 20);


    println!("My first name is {} and my family name is {} and Im {} years old!", person.0, person.1, person.2);

}