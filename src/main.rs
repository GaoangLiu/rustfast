use rustfast::File;
use std::env;
use core::fmt::Debug;
use csv::Error;
use std::collections::HashMap;

fn say<T: Debug + std::fmt::Display>(text: T){
    File::say(text)
}

fn main() -> Result<(), csv::Error> {
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
    let mut hm = HashMap::new();
    hm.insert("a", 123);
    let items=vec!["abc", "eft"];
    let values=vec![10, 20];
    let _hm2:HashMap<_, _> = items.iter().zip(values.iter()).collect();
    for (k, v) in &_hm2 {
        println!("{}, {}", k,v)
    }
    for (k, v) in &_hm2 {
        println!("{}, {}", k,v)
    }
    hm.insert("a", 110);
    println!("{:?}", hm);

    // panic!("crush and burn");
    // csv reader https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html
    let s = File::read("/Users/slipperl/Downloads/iterget-urls.csv");
    let mut reader = csv::Reader::from_reader(s.as_bytes());
    for (i, r) in reader.records().enumerate() {
        if i == 0 {
            let r = r?;
            println!("{:?}", &r);
            println!("{}", &r[2]);
            let items:Vec<_> = r[2].split("/").collect();
            println!("{}", &items.last().unwrap().split(".").collect::<Vec<_>>().first().unwrap());
        }
    }
    // 2021-07-07 Line 86
    Ok(())
}
