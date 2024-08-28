use std::io::{Read, Write};
use std::fs::File;

// NOTES:
// -> Struct std::fs::File : An object providing access to an open file on the filesystem.
//           An instance of a File can be read and/or written depending on what options it was opened with.
//           Files are automatically closed when they go out of scope. 
//    
// -> Trait 'std::io::Read' : The Read trait allows for reading bytes from a source. 
//           Implementors of the Read trait are called ‘readers’.
// -> Trait std::io::Write : A trait for objects which are byte-oriented sinks.
//           Implementors of the Write trait are sometimes called ‘writers’.


fn main(){

    // Let's open a file to read, 
    // File.open() :  Attempts to open a file in read-only mode.
    // open() returns a 'Result'.
    // We need to handle the result to check if the file was open or not.
    
    {
        let f = File::open("non_existant_file.txt");
        match f {
            Ok(_file) => println!("File Opened!"),
            Err(error) => eprintln!("Error: {:?}" , error)
        }

    }

    // now lets create a file and write some text in it
    // File.create()  :  Opens a file in write-only mode.
    {
        let mut f = File::create("new_file.txt").expect("Error creating file");
        let file_created = f.write_all("This is the text for the new file. \n".as_bytes());

        match file_created {
            Ok(_) => println!("Data written to file successfully."),
            Err(e) => eprintln!("Error writing to file: {}", e),
        }
    
    }

    // now, this example will open a file and read the content of this file
    // if the file exists and there is some content in it,
    // in this example we will use the https://doc.rust-lang.org/std/fs/struct.File.html#method.read_to_string-1[read_to_string] 
    // method provided from the Read impl for File trait.
    {
        let  file = File::open("new_file.txt"); 
        let mut content = String::new();   // String to receive the read content, if some. 

        match file {
            Ok(mut f) =>  {
                let size_read = f.read_to_string(&mut content);
                println!("read {:?} bytes from file.", size_read.unwrap());
            } ,
            Err(error) => eprintln!("Error while openning file: {:?} ", error )
            
        }
        println!("READ: {:?} ", content);
    }


// Struct std::fs::OpenOptions 
//      Options and flags which can be used to configure how a file is opened.
//      This builder exposes the ability to configure how a File is opened and what operations are permitted
//      on the open file. The File::open and File::create methods are aliases for commonly used options using this builder.
//      Ref: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
// 
//      example:  let file = OpenOptions::new()
//                                      .read(true)
//                                      .write(true)
//                                      .create(true)
//                                      .open("foo.txt");

    // In order to ilustrate the use of OpenOptions, lets open a file to append content to it.
    // Using OpenOptions, you can customize the file access mode beyond just reading, writing, or appending. 
    // For example, you can open a file for both reading and writing, or create a file only if it doesn’t exist.
    {
        use std::fs::OpenOptions;
        let file = OpenOptions::new().append(true).open("new_file.txt");
        match file {
            Ok(mut f) => {
                let appended = f.write_all("New content appended here! \n".as_bytes());
                match appended {
                    Ok(_) => println!("Data appended to file successfully."),
                    Err(e) => eprintln!("Error appending to file: {}", e),
                }
            },
            Err(error) => eprintln!("Error appending to file: {} " , error)
        }
    }

    
// NOTE:  
//      The File struct does not buffer reads and writes. 
//      For efficiency, consider wrapping the file in a BufReader or BufWriter when performing 
//      many small read or write calls, unless unbuffered reads and writes are required.  
//      When dealing with large files in Rust, it's important to handle them efficiently to avoid excessive memory usage 
//      and ensure that your program runs smoothly. Instead of reading or writing the entire file at once, 
//      which can consume a lot of memory, the best practice is to process the file in smaller chunks.

    // Let's see how to use BufReader and BufWriter to read and write large files. 
    // Let's start by creating a large file. 
    {
        use std::io::BufWriter;

        // this is a vector of dummy data with 10 mega bytes
        let data = vec![ 0u8; 1024];   
        // lets open a file in write only mode, it  gives us either a Result with a file inside or an error.
        let file = File::create("new_large_file.txt");
        match file {
            Ok( f) => {
                // file open: Lets create a buffered writer for efficient writting
                let mut buf_writer = BufWriter::new(f);
                // Define the chunck size to write to the buffer at each step
                let chunk_size = 1024;   // for instance 1 Kbyte
                // lets write chunck by chunck 
                for chunk in data.chunks(chunk_size) {
                    // Returns an iterator over chunk_size elements of the slice at a time, 
                    // starting at the beginning of the slice.  The chunks are slices and do not overlap. 
                    // If chunk_size does not divide the length of the slice, then the last chunk will 
                    // not have length chunk_size.
                    buf_writer.write_all(chunk).expect("Error writting to buffered writer");

                }
                println!("Create new large file! ");
            },
            Err(error) => eprintln!("Error creating a file: {} " , error)
        }
    }


    // Now, lets try to read a large file with Buffered Reader 
    // similarly to Buffered writer we need to wrap the reader in a BufReader
    {
        use std::io::{BufReader};
        println!("Trying to read a large file");   
        let file = File::open("new_large_file.txt").unwrap_or_else(|error|{  panic!("error: {} ", error)});
        // here in the previous line Im handling RECOVERABLE ERRORS by reducing the number of required 'match' with 'closures' and 'unwrap_or_else' method"
        // check handling_errors examples for more information about it. 
        
        let mut buf_reader = BufReader::new(file); 
        let mut buffer = [0; 32];   // size of buffer, in this case 4 bytes
        loop{            
            let read_result = buf_reader.read(&mut buffer);
            match read_result {
                Ok(size) => {
                    if size == 0 {
                        break; // Exit loop when file is fully read
                    }
                    println!("Bytes in the buffer => {:?}", &buffer[..size]);
                },
                Err(e) => {
                    eprintln!("error reading file: {}", e);
                    break;
                }
            }
        }
    }

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


}