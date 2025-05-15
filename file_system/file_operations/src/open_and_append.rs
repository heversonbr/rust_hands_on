use std::fs::OpenOptions;
use std::io::Write;

pub fn open_and_append(filename: &str, content: &str){
    // 4) OPEN to WRITE and APPEND 
    // In order to ilustrate the use of OPENOPTIONS, lets open a file to append content to it.
    // Using OpenOptions, you can customize the file access mode beyond just reading, writing, or appending. 
    // For example, you can open a file for both reading and writing, or create a file only if it doesn’t exist.
    println!("----------\nExample 4: Using Struct 'std::fs::OpenOptions': to customize the access mode.");
    println!("Trying to open the file {} in APPEND MODE", filename);
    match OpenOptions::new().append(true).open(filename) {
        Ok(mut f) => {
            println!("File Opened in append mode!, trying to append data...");
            match f.write_all(content.as_bytes()) {
                Ok(_) => println!("Data appended to file successfully."),
                Err(e) => eprintln!("Error appending to file: {}", e),
            }
        },
        Err(error) => eprintln!("Error appending to file: {} " , error)
    }
    // We can use the example 3 again just to read the new content
}