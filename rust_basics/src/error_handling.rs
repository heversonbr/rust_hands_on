// This example shows the basics of error handling in Rust
//
// Example based on the Online Rust Book!
//
// The Result enum is defined as having two variants, Ok and Err
//      enum Result<T, E> {
//          Ok(T),
//          Err(E),
//      }
// The T and E are generic type parameters, 
//  - 'T' represents the type of the '' that will be returnedvalue in a success case within the 'Ok' variant, and 
//  - 'E' represents the 'type of the error' that will be returned in a failure case within the 'Err' variant
// 

use std::{fs::File, io::{self, ErrorKind, Read}};

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn run(){

    // Example 1: UNRECOVERABLE ERRORS with 'panic!()' macro.
    // First lets see the macro 'panic!()', 
    // which can be used when our program fails in an unrecoverable fashion 
    // and can't handle the error gracefully. 
    {
        println!("Example 1: UNRECOVERABLE ERRORS with panic! ");
        // the macro will make the program panic and leave. 
        // uncomment the next line 
        // panic!("The program just panic here!");  // any code following this expression is unreachable

        // If we add the following line, the compiler wont let us compile the code: 'unreachable statement' 
        // because the next line will be unreachable (uncomment the next line to check)
        // println!("Test: this line will not show!");

    }
    
    // Example 2: Using a panic! with Backtrace.
    // This next example show a bit of more details about panic 
    // and about the message : "run with `RUST_BACKTRACE=1` environment variable to display a backtrace" 
    // check how the error message changes if we run with "RUST_BACKTRACE=1 cargo run"
    // note that a more detail information will be given if the compiler does not let you to compile  
    {
        println!("Example 2: UNRECOVERABLE ERRORS with Backtrace");

        a();

        fn a(){
            b();
        }

        fn b(){
            c();
        }

        fn c(){
            // NOTE: set number to 10 to see it panic 
            // it must panic with the message => thread 'main' panicked at src/error_handling.rs:56:17:
            let number = 11;
            if number == 10 { 
                panic!("I panic with number=10!");
            }
            println!("Example 2: I didnt panic!");
        }
    }

    // Example 3: lets check the RECOVERABLE ERRORS, 
    // the errors that we can handle gratefull, for these cases we use the 'RESULT' enum. 
    // the Result enum represents success or fail, 
    // just like the option enum, the Result enum has two variances:  'Ok' and 'Err'. 
    // 'Ok'  represents the success case and contains a values, and 
    // 'Err' represents the error case (and stores some generic error value). 
    //
    // Result differs from Option from the fact that WE USE RESULT USUALLY WHEN DIFFERENT TYPES OF ERRORS CAN HAPPEN. 
    // Some test can fail with many different reasons, not only because it is a 'None' type. 
    //
    // Functions return Result whenever errors are expected and recoverable. 
    // 
    {
        println!("Example 3: RECOVERABLE ERRORS");

        let f = File::open("hello.txt");
        // NOTE: an error will happen because the file 'hello.txt' does not exist! 
        // must fail with message: "No such file or directory"
        // this example will output the following:
        //      thread 'main' panicked at src/error_handling.rs:65:17:
        //      Failed while opening the file Os { code: 2, kind: NotFound, message: "No such file or directory" } 
        //      note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        // which makes sense 'cause the file hello.txt does not exist.

        //let f = match f {
        //    Ok(file) => file,
        //    Err(error) => {
        //        panic!("Failed while opening the file, Err:  {:?} ", error)
        //    },
        //};
    } 
    

    // Example 3.1: Matching on Different Errors
    //              let's adapt the previous example and make it create a new file, if it does not exist.
    //              in order to do that we will use an enum called 'ErrorKind' from std::io
    // NOTE: the previous example must be commented in order to this one to run (due to the panic at line 93)
    {
        println!("Example 3.1 : RECOVERABLE ERRORS: Matching on Different Errors");

        let f = File::open("another_hello.txt");
        
        let f: File = match f {

            Ok(file) => file,
            Err(error) => match error.kind() {
                // if an error happen, we try to match the error, 
                ErrorKind::NotFound => match File::create("another_hello.txt") {
                    // if the error is an ErrorKind::NotFound, we will create the file
                    Ok(fc) => fc ,
                    Err(err) => panic!("Error creating file: {:?}" , err)
                },
                any_other => { 
                     // if the error is another type, other than 'ErrorKind::NotFound' , we panic! 
                    // remind from enums: the last pattern will match all values not specifically listed! 
                    panic!("Problem opening file: {:?}" , any_other);
                }
            },
        };
        println!("File f: {:?}", f);
        
        
    }

    
    // Example 3.2: Reduce the number of matches by using 'closures' and 'unwrap_or_else' method.
    //
    // In the previous example, we have seen a lot of 'matches'! 
    // The match expression is very useful, but also very much primitive!
    // In this example we are going to reduce the number of required 'match' statements.
    //
    {
        println!("Example 3.2 : RECOVERABLE ERRORS: reducing the number of required 'match' with 'closures' and 'unwrap_or_else' method");
        // the unwrap_or_else method: Returns the contained 'Ok' value or computes it from a 'closure'
        // if it can't unwrap the Ok, the closure will receive the error and check the type of error.
        // inside the closure, if the error is an 'ErrorKind::NotFound' , we create the file. 
        let myfile = File::open("yet_another_file.txt").unwrap_or_else( |error| {
            if error.kind() == ErrorKind::NotFound {
                // When we try to create the file, we use the same approach with 'unwrap_or_else'
                // if we cannot create the file, another closure will receive the error and panic with this error message. 
                File::create("yet_another_file.txt").unwrap_or_else( |error| {
                    panic!("Problem creating file {:?} ", error);  
                })
            } else {
                panic!("Problem opening file {:?} ", error);
            }
        });
        println!("File myfile: {:?}", myfile);
    }


    // NOTE 1: 
    // SHORTCUTS for Panic on Error: 'UNWRAP' and 'EXPECT'
    //      The 'unwrap' method is a shortcut method 
    //      If the Result value is the 'Ok' variant, 'unwrap' will return the value inside the Ok,
    //      If the Result is the 'Err' variant, unwrap will call the panic!
    //      Ex: let greeting_file = File::open("hello.txt").unwrap();
    //
    //      We can use 'expect' in the same way as 'unwrap': to return the file handle or call the panic! macro. 
    //      The error message used by 'expect' in its call to panic! will be the parameter that we pass to 'expect', 
    //      rather than the default panic! message that unwrap uses. 
    //      Ex: let greeting_file = File::open("hello.txt").expect("hello.txt couldn't be open");

    // NOTE 2:
    // Example 4: PROPAGATING Errors 
    //            When a function’s implementation calls something that might fail, 
    //            instead of handling the error within the function itself, 
    //            you can RETURN THE ERROR TO THE CALLING CODE so that it can decide what to do. 
    //            This is known as PROPAGATING THE ERROR and gives more control to the calling code, 
    //            where there might be more information or logic that dictates 
    //            HOW the error should be handled than what you have available 'in the context of your code'.
    {
        #[allow(dead_code)]
        fn read_word_from_file(filename: &str) -> Result<String, io::Error> {

            println!("Example 4: PROPAGATING ERRORS");

            let file_handler = File::open(filename);
            
            let mut file_handler = match file_handler {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut read_word = String::new();

            match file_handler.read_to_string(&mut read_word) {
                Ok(_) => Ok(read_word),
                Err(e) => Err(e),
            }
        }

        // NOTE: 
        // The code that calls this function will get either 
        // - an Ok value that contains a word read from the file, or 
        // - an Err value that contains an io::Error. 
        // It’s up to the calling code to decide what to do with those values. 
        // We don’t have enough information on what the calling code is actually trying to do, 
        // so we PROPAGATE any 'success' or 'error' to the caller to handle it. 
    }

    // Example 5: Shortcut for PROPAGATING errors with operator '?' 
    //            This example shows how to use the operator '?' to propagate the errors back to the calling code,
    //            as shown in the previous example. 
    {
        #[allow(dead_code)]
        #[allow(unused_variables)]
        fn read_word_from_file(filename: &str) -> Result<String, io::Error> {

            println!("Example 5: shortcut for PROPAGATING ERRORS with operator '?'");
            
            let mut read_word = String::new();
            let file_handler = File::open(filename)?.read_to_string(&mut read_word)?;
            Ok(read_word)
        }

    // Explanation: 
    // The operator '?' placed after a 'Result' value is defined to work in almost the same way as the match expressions.
    // If the value of the 'Result' is an Ok, the value inside the Ok will get returned from this expression, and the program will continue.
    // If the value is an 'Err', the 'Err' will be returned from the whole function as if we had used the 'return' keyword so the error value will be propagated to the calling code.
    }

    // NOTE: Reading a file into a string is a fairly common operation,
    // so the standard library provides the convenient 'fs::read_to_string' function 
    // that opens the file, creates a new String, reads the contents of the file, 
    // puts the contents into that String, and returns it. 
    // Of course, using fs::read_to_string doesn’t give the opportunity to explain 
    // all the error handling that happens behind the scenes.


    // So how do you decide when you should call panic! and when you should return Result? 
    // 
    // When code panics, there’s no way to recover.
    // You could call panic! for any error situation, whether there’s a possible way to recover or not, 
    // but then you’re making the decision that a situation is unrecoverable on behalf of the calling code.
    //
    // When you choose to return a Result value, you give the calling code options. 
    // The calling code could choose to attempt to recover in a way that’s appropriate for its situation, 
    // or it could decide that an Err value in this case is unrecoverable, 
    // so it can call panic! and turn your recoverable error into an unrecoverable one. 
    // Therefore, returning Result is a good default choice when you’re defining a function that might fail.
    //
    // In situations such as examples, prototype code, and tests, it’s more appropriate to write code that panics instead of returning a Result.
    // When you’re writing an example to illustrate some concept, also including robust error-handling code can make the example less clear.
    //
    // It’s understood that a call to a method like 'unwrap' (that could panic) is meant as a 'placeholder' for the way you’d want your application to handle errors.
    // The 'unwrap' and 'expect' methods are very handy when prototyping, before you’re ready to decide how to handle errors.

}   