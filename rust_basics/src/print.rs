// 'pub' makes the function accessible publicly 
// this example shows a glimpse of printing/formatting in Rust
#[allow(dead_code)]

pub fn run() {

    // println! is a macro that prints text to the console.
    // Sends the output to the standard output (stdout)
    // Typically used for general output that the user expects to see as normal program operation. 
    println!("Hello printing from file : print.rs");

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

    // eprintln! 
    // eprintln! is another macro used to print text to the console.
    // Typically used for error messages or diagnostics that should be separated from regular output, 
    // especially when you want to distinguish between normal output and error conditions.
    // Sends the output to the standard error (stderr) 
    // Example:  If we run the following program , like this: 'cargo run > output.log 2> error.log'
    // fn main() {
    //      println!("This is a normal output.");
    //     eprintln!("This is an error message.");
    // }
    //  The "This is a normal output." will be stored in output.log, 
    //  and the error message "This is an error message." will be stored in error.log

    eprintln!("This message is sent to the standard error (stderr).");

}