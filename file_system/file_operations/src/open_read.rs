use std::fs::File;

pub fn open_to_read(filename:  &str) {

    // 1) EXISTING FILE: OPEN READ-ONLY
    //   Let's open a file to read, 
    //   File.open() :  Attempts to open a file in read-only mode.
    //   open() returns a 'Result'.
    //   We need to handle the result to check if the file was open or not.
    
    println!("----------\nExample 1: Just Trying to open a file (read-only): ");
    {

        match File::open(filename) {
            Ok(_file_opended) => println!("File {} was successfully opened!", filename),
            Err(error) => eprintln!("Error: {:?}" , error)
        }
    }
    // in this example, the 'non_existant_file.txt' initially does not exist. 
    // so it will return an error.

}