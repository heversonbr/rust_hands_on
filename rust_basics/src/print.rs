// pub make the function accessible publicly 
// this file shows a bit of printing formatting using rust
#[allow(dead_code)]

pub fn run() {

    // println! is a macro that prints text to the console.

    // Print to console 
    println!("Hello printing from another file's function: print.rs");

    // Basic formatting
    println!("Printing using placeholders :  {} is {} !", "Rust", "Fun");

    // Positional arguments: 
    println!("Printing using positional arguments : {0} is an {1} language and I {2} it !", "Rust", "interesting", "like");
    println!("Printing using positional arguments : {2} is an {1} language and I {0} it !", "Rust", "interesting", "like");

    // Named arguments:
    println!{
        "Printing using named arguments : {language} is an {opinion} programming language and I {feel} it!",
        language="Rust",
        opinion="interesting",
        feel="like"
    }

    // Placeholder traits:
    println!("Printing using traits. 20 in Binary: {:b} , Hex: {:x} , Octal: {:o} ", 20 , 20 , 20);

    // Placeholder for debug traits:
    println!("Printing using debug traits: {:?}" , (12, true, "hello"));

    // Basic Math
    println!("Printing using basic math:  10 + 10 = {}", 10 + 10);
}