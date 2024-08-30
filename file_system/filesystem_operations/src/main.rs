// Module std::fs
//      Filesystem manipulation operations.
//      This module contains basic methods to manipulate the contents of the local 
//      filesystem. All methods in this module represent cross-platform filesystem 
//      operations. Extra platform-specific functionality can be found in the extension
//      traits of std::os::$platform.
// Ref: https://doc.rust-lang.org/std/fs/index.html 

use std::fs;

// In this example we are interested in the 'ReadDir' struct 
// which is an 'Iterator over the entries' in a directory.
// this Iterator can be obtained by invoking the function 'read_dir' from std::fs

fn main() {

    // read_dir() : returns a Result<ReadDir, Error>, it expects to receive a path. 
    let read_dir_result = fs::read_dir(".");
    // handling the Result with a match pattern (there are many different ways of doing this)
    match read_dir_result {
        Ok(read_dir_iterator) => {
            // if Result is Ok, then we have the ReadDir Iterator, over the entries in the directory
            // we can handle this Iterator in many different ways, 
            // in this example, we are just iterating over each element to print. 
           for dir_entry in read_dir_iterator {
                let path = dir_entry.as_ref().unwrap().path();
                // Returns the full path to the file that this entry represents.
                // The full path is created by joining the original path to read_dir with the filename of this entry.
                let metadata = &dir_entry.as_ref().unwrap().metadata();
                // Returns the metadata for the file that this entry points at.
                let filetype = &dir_entry.as_ref().unwrap().file_type();
                // Returns the file type for the file that this entry points at.
                let filename = &dir_entry.as_ref().unwrap().file_name();
                // Returns the file name of this directory entry without any leading path component(s).

                println!("path: {:?} \n 
                          metadata: {:?} \n 
                          filetype: {:?} \n 
                          filename: {:?} \n" , 
                          path, metadata, filetype, filename);
           }
        },
        Err(error) => {eprintln!("error : {}" , error);}
    }
    
}

