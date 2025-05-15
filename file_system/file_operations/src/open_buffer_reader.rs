use std::io::BufReader;
use std::fs::File;
use std::io::Read;

// -----------------------------------------------------------------------------------
// NOTE:  
// The Struct (std::fs::File;) does not buffer reads and writes. 
// For efficiency, consider wrapping the file in a BufReader or BufWriter when performing 
// many small read or write calls, unless unbuffered reads and writes are required.  
// When dealing with large files in Rust, it's important to handle them efficiently to
//  avoid excessive memory usage and ensure that your program runs smoothly. Instead of reading 
// or writing the entire file at once, which can consume a lot of memory, the best practice 
// is to process the file in smaller chunks.
// -----------------------------------------------------------------------------------
// Let's see how to use BufReader to read and write large files. 
// similarly to Buffered writer we need to wrap the reader in a BufReader

pub fn buffer_reader_example(filename: &str){
    
    println!("----------\nExample 6: using BufReader] to read large files (for efficiency and low memory consumption)");
    println!("Trying to open the large file: {}.", filename);   

    let file = File::open(filename).unwrap_or_else(|error|{ panic!("error: {} ", error)});
    // the previous line weIm handle RECOVERABLE ERRORS by reducing the number of 
    // required 'match' statements by using  'unwrap_or_else' method and a 'closure' (see handling_errors for more)
    
    println!("Large file [new_large_file.txt] opened!");
    println!("Creating a BufReader from the open file");
    
    let mut buf_reader = BufReader::new(file); 
    let mut buffer = [0; 32];   // size of buffer to read, in this case 4 bytes
    let mut total_bytes_read = 0;  // Counter for total bytes read

    println!("reading bytes from the file: {} bytes at a time (size of buffer).", buffer.len()); 
    loop{            
        match buf_reader.read(&mut buffer) {
                // read() method : read(self, buf: &mut [u8]) -> Result<usize>
                // Pull some bytes from this source into the specified buffer, 
                // returning how many bytes were read.
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break; 
                    // Exit loop when file is fully read
                }
                total_bytes_read += bytes_read;
                //println!("bytes in the buffer:  {:?}", &buffer[..size]);
            },
            Err(e) => {
                eprintln!("error reading file: {}", e);
                break;
            }
        }
    }

    println!("Total Bytes Read: {}" , total_bytes_read); 

}

// Other useful methods:
//
// - read_to_end(buf: &mut Vec<u8>) -> Result<usize>
//   provided by the Read Trait
//   read all bytes until EOF in this source, placing the read bytes into the buffer passed as parameter.
//   All bytes read from this source will be appended to the specified buffer buf.
//   This function continuously call read() to append more data to buf until 
//   read() returns either Ok(0) or an error.