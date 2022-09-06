// this file shows a bit how to use variables in rust
// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language
#[allow(dead_code)]

pub fn run(){

    println!("Trying variables! \n Remind: Variables are immutable by default");
    // Rust provides type safety via static typing. Variable bindings can be type annotated when declared. 
    // However, in most cases, the compiler will be able to infer the type of the variable from the context, 
    // heavily reducing the annotation burden. Values can be bound to variables, using the let binding. Ex:
    let language = "Rust";

    println!("I'm learning {language} programming!" );
    // Mutability: Variable bindings are immutable by default, but this can be overridden using the mut modifier.
    // Ex: we cannot change the variable 'language' like this =>  language = "javascript"
    //     we have to declare mutable 'mut', in order to change the value of a variable.
    let mut status = "learning"; 
    println!("I'm {status} {language} programming!");
    status = "dominating";
    println!("I'm {status} {language} programming!");

    // NOTE: The compiler forbids use of uninitialized variables!

    // We can also declare a constant
    // when declaring constants we have to explicitly declare its type. here we will declare an integer 32bits
    const ID:i32 = 0010;
    println!("ID is a constant declared as [ID:i32 = 0010] (integer, 32 bits) and its value is : {}", ID);

    // Assigning multiple variables
    let (action, language, felling) = ("learning","Rust", "like");
    println!("Assigning multiple variables at a time: I'm {} {} programming and I {} it", action, language,felling);

}