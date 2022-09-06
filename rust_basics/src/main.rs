/// # RUST Hands On
/// 
/// This is a very basic hands on that drives me in my first steps with RUST.
/// Below you will find modules imports and their corresponding files.
/// In order to run an example, just uncomment one of the import files and 
/// its corresponding execution in the `main()` function. 
/// 
/// 
/// Example: In order to run the vector example.
/// 
/// - Uncomment `mod vectors`
/// - Uncoment the function call `vectors::run();` in the main function.

// import modules=>  mod <path/module_name>
#[allow(dead_code)]

mod print;
mod vars;
mod types;
mod strings;
mod arrays;
mod vectors;
mod tuples;
mod conditionals;
mod loops;
mod functions;
mod point_ref;
mod structs;
mod enums;
mod cli;

mod ownership;
mod references;

fn main()  {
   
    // executes the function run from module print. Print module shows how to print and format what we want to print
    //print::run();
    //vars::run();
    //types::run();
    // strings::run();
    //tuples::run();
    //arrays::run();
    //vectors::run();
    //conditionals::run();
    // loops::run();
    //functions::run();
    //point_ref::run();
    //structs::run();  
    // enums::run();
    //cli::run();

    //ownership::run();
    references::run();

}