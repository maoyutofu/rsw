use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn create_not_exists(dir: &str) {
    if !Path::new(dir).exists() {
        match fs::create_dir_all(dir) {
            Err(why) => panic!("create {}: {}", dir, why.description()),
            Ok(_) => println!("create {}", dir),
        };
    }
}

pub fn write_file(file_name: &str, content: &str) {
    let index_md = Path::new(&file_name);
    let mut file_index_md = match File::create(index_md) {
        Err(why) => panic!("create {}: {}", file_name, why.description()),
        Ok(file) => file,
    };
    match file_index_md.write_all(content.as_bytes()) {
        Err(why) => panic!("write {}: {}", file_name, why.description()),
        Ok(_) => println!("write {}", file_name),
    };
}