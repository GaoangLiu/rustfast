use core::fmt::Debug;
use std::fs;
use std::path::Path;
use std::fs::metadata;
pub struct File {}

impl File {
    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    pub fn say<T: Debug + std::fmt::Display>(text: T) {
        println!("{}", text);
    }

    pub fn vecsay<T: Debug>(vec: &Vec<T>) {
        println!("{:?}", vec);
    }

    pub fn read(path: &str) -> String {
        if !File::exists(path) {
            println!("File {} not exists", path);
            return "".to_string();
        }
        fs::read_to_string(path).expect("Something went wrong reading the file")
    }

    pub fn write(contents: &str, path: &str) {
        fs::write(path, contents).expect("Unable to write file");
    }
    pub fn remove(path: &str) {
        fs::remove_file(path).expect("Unable to remove file");
    }

    pub fn walk(path: &str) -> Vec<&str> {
        let mut ans = vec![];
        let md = metadata(path).unwrap();
        if md.is_file() {
            ans.push(path); 
        } else {
            1;
        }
        ans
    }
}

pub fn say<T: Debug + std::fmt::Display>(text: T) {
    println!("{}", text);
}

pub fn put<T: Debug + std::fmt::Display>(text: T) {
    print!("{}", text);
}
