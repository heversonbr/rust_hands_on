use std::fs::File;
use std::io::Read;


pub fn open_and_read(filename: &str) {

    // 3) OPEN EXISTING FILE AND READ CONTENT 
    //  this example will open a file and reads the contents of a file into a String
    println!("----------\nExample 3: Open a file {} to read: (using read_to_string method)", filename);
    let mut content: String = String::new();   // String to receive the read content, if some. 
    match File::open(filename) {
        Ok(mut f) =>  {
            println!("File [new_file.txt] Opened!");
            let size_read = f.read_to_string(&mut content);
            println!("read {:?} bytes from file.", size_read.unwrap());
        } ,
        Err(error) => eprintln!("Error while openning file: {:?} ", error )
    }
    println!("CONTENT READ: \n {:?} ", content);
    // method raed_to_string()
    // provided by the Read Trait
    // sintax:  `read_to_string(self, buf: &mut String)`
    // Reads all bytes until EOF in this source, appending them to the String buffer passed as parameter.
    // If successful, this function returns the number of bytes which were read and appended to buf.
    // If the data in this stream is not valid UTF-8 then an error is returned and buf is unchanged.
    //
    // ref: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
    //       
    // in this example we used https://doc.rust-lang.org/std/fs/struct.File.html#method.read_to_string-1[read_to_string] method provided from the Read impl for File trait.

}