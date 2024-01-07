// build out recursive function that enables step into nested directories

use std::path::PathBuf;

// add in posix compliance with use std::process::{Command, Stdio}; so output can be sent to the next command

#[derive(Debug)] // need to look into this macro further
struct FileInfo {
    path: PathBuf,
    hash: String,
    //datemodified: String, // add this so one of the options can be a delete stdout of the newest files
}
// add support for stdin and arguments 
fn main() {
    let mut storage = Vec::new();
    let dir = "/home/mac/pix/test";
    let data = std::fs::read_dir(dir).unwrap();
    // returns "ReadDir("/home/mac/pix/wall")" how the hell do I get inside the iterator?? - a method
    for item in data {
        // dealing with errors
        let item_safe = match item {
            Ok(item) => item,
            Err(e) => panic!("error occured: {}",e),    
        };

        let file_data = std::fs::read(item_safe.path()).unwrap();
        //if entry.
        // need to hash the fuction
        let file_data_hash = sha256::digest(file_data); // super performant... should probably switch this for something easier
        //println!("File path: {:?} - Hash: {:?}",item_safe.path(),file_data_hash);
        // initailization of the struct
        let hash_data = FileInfo {
            path: item_safe.path(),
            hash: file_data_hash,
        };
        storage.push(hash_data);
    }

    // if hash matches list file
    for i in 0..storage.len() as usize{
        for j in i..storage.len() as usize{
            if i != j && storage[i].hash == storage[j].hash{
                println!("Duplicate: {:?} and {:?}", storage[i].path, storage[j].path)
            }
        
        } 
    }
}
