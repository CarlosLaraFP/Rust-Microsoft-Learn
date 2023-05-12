use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;


pub fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut buffer = String::new();

    // Access a file at a specified path
    // - Pass variable to `file` variable on success, or
    // - Return from function early if there's an error
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error)
    };

    // Read file contents into mutable `String` variable with `read_to_string`
    // Return from function early if there's an error
    match file.read_to_string(&mut buffer) {
        Ok(size) => println!("Buffer size: {}", size),
        Err(io_error) => return Err(io_error)
    };

    Ok(buffer)
}
