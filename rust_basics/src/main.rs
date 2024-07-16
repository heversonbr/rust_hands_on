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
mod structs;
mod enums;
mod enum_option_ex1;
mod enum_option_ex2;
mod cli;
mod ownership;
mod references;
mod smart_pointers;
mod smart_pointers_box;
mod smart_pointers_rc;
mod raw_pointers;
mod options;
mod match_patterns;
mod simple_boxed_list;
mod error_handling;
mod iterators;
mod closures;
mod hashmaps;



fn main()  {
   
    // executes the function run from module print. Print module shows how to print and format what we want to print
    //print::run();
    //vars::run();
    //types::run();
    //strings::run();
    //tuples::run();
    //arrays::run();
    //vectors::run();
    //conditionals::run(); 
    //loops::run();
    //functions::run();
    //structs::run();  
    //enums::run();
    //enum_options_ex1::run();
    //enum_option_ex2::run();
    //options::run();
    //match_patterns::run();
    //cli::run();
    //ownership::run();
    //references::run();
    //smart_pointers::run();
    //smart_pointers_box::run();
    smart_pointers_rc::run();
    //simple_boxed_list::run();
    //raw_pointers::run();
    //error_handling::run();
    //iterators::test_iterators();
    //closures::test_closures();
    //hashmaps::run();
    
    

}
