use core::fmt::Debug;
use std::fs;
pub struct File {}

impl File {
    pub fn say<T: Debug + std::fmt::Display>(text: T) {
        println!("{}", text);
    }

    pub fn vecsay<T: Debug>(vec: &Vec<T>) {
        println!("{:?}", vec);
    }

    pub fn read(path: &str) -> String {
        fs::read_to_string(path).expect("Something went wrong reading the file")
    }
    pub fn write(contents: &str, path: &str) {
        fs::write(path, contents).expect("Unable to write file");
    }
    pub fn remove(path: &str) {
        fs::remove_file(path).expect("Unable to remove file");
    }
}
