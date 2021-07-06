use rustfast::File;
use std::env;
use core::fmt::Debug;

fn say<T: Debug + std::fmt::Display>(text: T){
    File::say(text)
}

fn main() {
    println!("rust fast it!");
    let f = File::read("Cargo.toml");
    println!("{}", f);

    File::write("Cargo.toml", "/tmp/rust.txt");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    File::say("Hello");
    File::say("Hello say");
    File::vecsay(&args);
    // 07-01-2021 Line 20

    println!("Iterate from one to three");
    for i in 0..3 {
        println!("{}", i);
    }

    for c in "abc".chars() {
        println!("{}", c);
    }

    let f = "RUST".to_string();
    for c in f.chars() {
        println!("{}", c);
    }

    let mut s=String::new(); 
    s.push_str("abc");
    println!("{}", s);
    let s2 = "abc"; 
    let s3 = s + &s2;
    println!("{}", &s3);
    println!("{}", s3);

    let hello = "aeiou";
    let slice = &hello[..3];
    File::say(slice);
    File::say(slice);
    for c in slice.bytes(){
        say(c);
    }
    // 2021-07-06 Line 53
}
