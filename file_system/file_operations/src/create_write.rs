use std::io::{Write};
use std::fs::File;

// 2) CREATE A FILE: OPEN WRITE-ONLY
    //    now lets create a file and write some text in it
    //  - File.create()  :  Opens a file in write-only mode.
    //  - write_all(): Attempts to write an entire buffer into this writer.
    //      This method will continuously call write until there is no more data 
    //      to be written or an error is returned provided by the 'Write' Trait
    pub fn create_and_write(filename: &str, content: &str) {

        println!("----------\nExample 2: Trying to create a new file and open it to write: ");
        match File::create(filename) {  
            // File::create() <- This function will create a file if it does not exist, and will truncate it if it does.
            // if you need create if it doesn't exist but open if exists, then use Struct OpenOptions with 'create(true)' method.
            Ok(mut file_opened) => {
                        println!("File successfully open");
                        match file_opened.write_all(content.as_bytes()) {
                            Ok(_) => println!("Data written to file successfully."),
                            Err(e) => eprintln!("Error writing to file: {}", e),
                        }
                    },
            Err(e) => eprintln!("Error writing to file: {}", e),
        }
    }