use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn open_and_read_by_line_w_iterator(filename: &str) {

    // 7) OPEN EXISTING FILE AND READ CONTENT LINE BY LINE
    //  this example will open a file and reads the contents of a file into a String
    println!("----------\nExample 7: Open a file {} to read line by line USING ITERATOR provided by method 'BufReader::lines()')", filename);
    
    let file = File::open(filename).unwrap_or_else(|error|{ panic!("error: {} ", error)});
    // the previous line weIm handle RECOVERABLE ERRORS by reducing the number of 
    // required 'match' statements by using  'unwrap_or_else' method and a 'closure' (see handling_errors for more)

    let buf_reader = BufReader::new(file);   // Wrap the file in BufReader

    println!("reading bytes from the file");
    // Iterate over each line in the file
    for line in buf_reader.lines() {   // <- Returns an iterator over the lines of this reader.
        match line {
                Ok(line) => { 
                    println!("Line =>  {}", line); 
                },  
                Err(e) => eprintln!("Error reading line: {}", e),  // Handle any read errors
        }
    }
}

// The read_line(&mut buf) method is another way to read a file line by line in Rust. 
// Unlike BufReader::lines(), which returns an iterator, read_line() reads one line at a time and appends it to a provided buffer. 
// It’s a more manual way of handling line-by-line reading but still very efficient.

pub fn open_and_read_by_line_no_iterator(filename: &str){ 

    println!("----------\nExample 7.1: Open a file {} to read line by line WITHOUT ITERATOR: uses 'BufReader::read_line()')", filename);
    let file = File::open(filename).unwrap_or_else(|error|{ panic!("error: {} ", error)});
    let mut buf_reader = BufReader::new(file); 
    let mut buf = String::new();  // <- Create a mutable string to hold each line

     // Read lines one by one using read_line
    loop{
        match buf_reader.read_line(&mut buf) {
            Ok(bytes_read) => {  
                if bytes_read == 0 {  // Exit loop when nothing else is read from the file by the BufReader
                    break; 
                }
                println!("read line => {}", buf.trim());  // Print the line (trim removes the trailing newline)
                buf.clear();  // Clear the buffer for the next line

            },
            Err(e) => {
                eprintln!("error reading file: {}", e);
                break;
            }
        }
    }
}