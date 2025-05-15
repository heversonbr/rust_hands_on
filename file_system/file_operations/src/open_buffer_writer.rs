use std::io::BufWriter;
use std::fs::File;
use std::io::Write;

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
    // Let's see how to use BufReader and BufWriter to read and write large files. 
    // Let's start by creating a large file. 


pub fn buffer_writer_example(filename: &str, data: &Vec<u8>){

    println!("----------\nExample 5: using BufWriter to write large files, for efficiency and low memory consumption.");   
   
    // this is a vector of dummy data with 10 mega bytes
    // we want to store this data in a file
   // let data = vec![0u8; 10_485_760]; // <- a vector of 10M Bytes

    // lets open a file in write only mode, it gives us either a Result with a file inside or an error.
    
    println!("Trying to create a new 'large' file [{}]", filename);
    let file = File::create(filename);
    match file {
        Ok( f) => {
            println!("Large file [new_large_file.txt] created!");
            // file is open: Lets create a buffered writer for efficient writting
            println!("creating a BufWriter for new_large_file.txt].");
            let mut buf_writer = BufWriter::new(f);
            // Define the chunck size to write to the buffer at each step
            // for instance 1 Kbyte (use a larger number if you want here, it is just an example)
            let chunk_size = 1024;   
            // lets write chunck by chunck 
            println!("writing {} bytes to file using the BufWriter", &chunk_size);
            for chunk in data.chunks(chunk_size) {
            // chuncks(): Returns an iterator over chunk_size elements of the slice at a time, 
            //            starting at the beginning of the slice.  The chunks are slices and do not overlap. 
            //            If chunk_size does not divide the length of the slice, then the last chunk will 
            //            not have length chunk_size.
                buf_writer.write_all(chunk).expect("Error writting to buffered writer");
            }
            println!("New large file Created!");
        },
        Err(error) => eprintln!("Error creating 'large' file: {} " , error)
    }
    

}

// -----------------------------------------------------------------------------------
// NOTES: Best Practices for Handling Large Files
//
// - 'Use Buffered I/O': Always use BufReader and BufWriter when dealing with large files to reduce the number of I/O operations, 
//                       which can be expensive.
// - 'Process in Chunks': Reading or writing in smaller chunks (e.g., 1 KB or 4 KB) helps manage memory usage effectively.
// - 'Iterate Over Lines': When the file is line-oriented (like a CSV or log file), read it line by line.
// - 'Error Handling': Properly handle errors using Result and the '?' operator 
//                     to ensure that your application can gracefully handle issues like file not found or permission denied.
// -> Even when working with small files, using buffered I/O is generally a good habit, 
// as it can prevent unnecessary I/O operations and improve overall performance.
// -> Be mindful of the scope to ensure files are closed as soon as you're done with them.
// -> Use OpenOptions when you need to customize file operations, such as appending to a file or creating a file if it doesn't exist.
// -> Keep File Operations Minimal: Open a file, perform the necessary operations, and close it as soon as possible.
// -> Consider Using serde for Serialization/Deserialization: If you're working with structured data (like JSON or CSV), 
//    consider using the serde crate to handle serialization and deserialization. 
//    This is particularly useful when reading and writing structured data files
// -----------------------------------------------------------------------------------