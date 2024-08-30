// We want to explore some the functionalities of the std::fs module 
// We want to search and read all '.gitignore' files in a given directory and all its subdirectories, 
// then, check if the '.gitignore' files have a specific entry: such as 'target/' and 'Cargo.lock'. 
// if not, add/append these entries in the .gitignore file 
// Obs: it shouldn't create any new gitignore file, just manipulate any existing ones.

//  NOTES:
//      std::path::Path:  
//      A slice of a path (akin to str). 
//      This type supports a number of operations for inspecting a path, including breaking the path into its components 
//      (separated by / on Unix and by either / or \ on Windows), extracting the file name, determining whether the path 
//      is absolute, and so on.

use std::{fs::{self, OpenOptions}, io::{self, BufRead, Write}, path::PathBuf}; 

fn find_files(current_path: &PathBuf, target_name: &str) -> io::Result<Vec<PathBuf>> {
    
    let mut found_files = Vec::<PathBuf>::new();
    for read_dir_iterator_result in fs::read_dir(current_path)? {
        let dir_entry = read_dir_iterator_result?;
        if dir_entry.path().is_dir() {
            found_files.extend(find_files(&dir_entry.path(), target_name)?);
        }else if dir_entry.path().is_file() && dir_entry.file_name() == target_name {
            found_files.push(dir_entry.path());
        }
    }
    Ok(found_files)
}

fn update_file(file_path: &PathBuf, target_name: &str) -> io::Result<()> {
  
    // We could open as follows,
    // if let Ok(file) =  OpenOptions::new().read(true).write(true).open(file_path) {}
    //  but it does not propagates the error back to the function call as it does with '?' operator, lets open like this:
    println!("Checking file at: {}" , &file_path.to_str().unwrap());
    let file = OpenOptions::new().read(true).write(true).append(true).open(file_path)?;
    
    let metadata = file.metadata()?;
    if metadata.len() == 0 {  // file is empty, we dont need to read it
        println!("File is empty, writing {} to it", target_name);
        let mut buf_writer: io::BufWriter<&fs::File> = io::BufWriter::new(&file);
        buf_writer.write_all((target_name).as_bytes())?;
        return Ok(());
    }
    // Get a BufferReader
    let buf_reader = io::BufReader::new(&file);

    //  NOTE: As we are reading a text file and Im interested in each like of the file .gitignore
    // We will read the text file line by line, there are different ways of implementing it.
    // for instance, we can use one of the 2 methods implemented by BufRead:  read_line() or lines()
    // -----------------------
    // read_line(buffer: &mut String):  Read all bytes until a newline (the 0xA byte) is reached, and append them to the provided String buffer.
    //                                  Previous content of the buffer will be preserved. To avoid appending to the buffer, you need to clear it first.
    // lines(self) : Returns an iterator over the lines of this reader.
    //               Each string returned will not have a newline byte (the 0xA byte) or CRLF (0xD, 0xA bytes) at the end. 
    // -----------------------
    // here is a solution with read_line(): 
    //let mut line = String::new();
    //let mut found = false;
    //while let Ok(_bytes_read) = buf_reader.read_line(&mut line){
    //     if _bytes_read == 0 {break;}
    //     if line == target_name{
    //         found = true;
    //     }
    //     line.clear();
    //}
    //if !found{
    //     let mut buf_writer: io::BufWriter<&fs::File> = io::BufWriter::new(&file);
    //     buf_writer.write_all((String::from("\n") + target_name).as_bytes())?;
    //}
    //
     // -----------------------
    // here is a solution with lines(): 
    // let mut found = false;
    // for line_result in buf_reader.lines(){
    //     let line = line_result?;
    //     if line == target_name {
    //         found = true;
    //     }
    // }
    // if !found {
    //     let mut buf_writer: io::BufWriter<&fs::File> = io::BufWriter::new(&file);
    //     buf_writer.write_all((String::from("\n") + target_name).as_bytes())?;
    // }
    // -----------------------
    // the previous can be improved by using Iterator methods such as maps, filters, etc:
    // lets see how to use Iterator's methods to improve the previous solution.
    let found = buf_reader.lines() 
                // get an iterator from the buffer reader
                .filter_map(Result::ok) 
                // The returned iterator yields only the values for which the supplied closure returns Some(value).
                // in this case filters the lines that have an Ok() result, 
                .inspect(|current_line| println!("Inspecting (found): {}", current_line )) 
                // Does something with each element of an iterator (in this case only print the current line)
                .any(|line|  line == target_name);
                // Check if any line matches the target_name
                // any() takes a closure that returns true or false. It applies this closure to each element of the iterator, 
                // and if any of them return true, then so does any(). If they all return false, it returns false.
    
    if !found {  // if the target name is not found in any of the lines we write it to the file
        let mut buf_writer: io::BufWriter<&fs::File> = io::BufWriter::new(&file);
        buf_writer.write_all((String::from("\n") + target_name).as_bytes())?;
    }

    Ok(())

}

fn main() {
    
    
    // -----------------------------------
    // target FILE to search
    let target_name = ".gitignore";
    // base directory to start searching
    //let base_root_path = PathBuf::from( ".");
    let base_root_path = PathBuf::from( ".");
    // -----------------------------------
    // target TEXT STRING to search in each file, create it if not found
    //let target_string = "Cargo.lock";
    let target_string = "target/";
    // -----------------------------------

    let mut found_files:Vec<PathBuf> = Vec::<PathBuf>::new();
    match find_files(&base_root_path, &target_name) {
        Ok(found) => found_files.extend(found),
        Err(e) => eprintln!("{:?}" , e)
    }
    println!("{:?}", found_files);

    for file_path in found_files{
        let _res = update_file(&file_path, target_string);
    }


}

