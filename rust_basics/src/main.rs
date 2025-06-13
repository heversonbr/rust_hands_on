mod arrays;
mod cli;
mod closures;
mod conditionals;
mod enum_option_ex1;
mod enum_option_ex2;
mod enums;
mod error_handling;
mod functions;
mod hashmaps;
mod iterators;
mod loops;
mod match_patterns;
mod options;
mod ownership;
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
mod raw_pointers;
mod references;
mod simple_boxed_list;
mod smart_pointers;
mod smart_pointers_arc;
mod smart_pointers_box;
mod smart_pointers_rc;
mod smart_pointers_refcell;
mod strings;
mod structs;
mod tuples;
mod types;
mod vars;
mod vectors;

fn main() {
    // For each test we eill execute the function run() at the specific module.
    // For examples the 'print' module shows how to print and format
    // below each module treats a specific basic topic

    //arrays::run();
    //cli::run();
    //conditionals::run();
    closures::run();
    //enums::run();
    //enum_options_ex1::run();
    //enum_option_ex2::run();
    //error_handling::run();
    //functions::run();
    //hashmaps::run();
    //1iterators::run();
    //loops::run();
    //match_patterns::run();
    //options::run();
    //ownership::run();
    //print::run();
    //raw_pointers::run();
    //references::run();
    //simple_boxed_list::run();
    //strings::run();
    //smart_pointers::run();
    //smart_pointers_box::run();
    //smart_pointers_rc::run();
    //smart_pointers_arc::run();
    //smart_pointers_refcell::run();
    //structs::run();
    //tuples::run();
    //types::run();
    //vars::run();
    //vectors::run();
}
