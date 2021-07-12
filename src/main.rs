mod lib;
use clap::{App, Arg};
use core::fmt::Debug;
use rand::Rng;
use rustfast::File;
use std::collections::HashMap;
use std::env;
use std::io;
use std::fs::{self, DirEntry};
use std::process::Command;
use std::time::{Duration, Instant};

fn say<T: Debug + std::fmt::Display>(text: T) {
    File::say(text)
}

fn main() -> io::Result<()>{
    let start = Instant::now();
    println!("rust fast it!");
    if File::exists("Cargo.toml") {
        let f = File::read("Cargo.toml");
        println!("{}", f);
    }

    if File::exists("/tmp/rust.txt") {
        File::write("Cargo.toml", "/tmp/rust.txt");
    }

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    File::say("Hello");
    File::say("Hello say");
    File::vecsay(&args);
    // 07-01-2021 Line 20

    println!("Iterate from one to three");
    for i in 0..3 {
        print!("{}", i);
    }

    for c in "abc".chars() {
        print!("{}", c);
    }

    let f = "RUST".to_string();
    for c in f.chars() {
        print!("{}", c);
    }

    let mut s = String::new();
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
    for c in slice.bytes() {
        say(c);
    }
    // 2021-07-06 Line 53
    let mut hm = HashMap::new();
    hm.insert("a", 123);
    let items = vec!["abc", "eft"];
    let values = vec![10, 20];
    let _hm2: HashMap<_, _> = items.iter().zip(values.iter()).collect();
    for (k, v) in &_hm2 {
        println!("{}, {}", k, v)
    }
    for (k, v) in &_hm2 {
        println!("{}, {}", k, v)
    }
    hm.insert("a", 110);
    println!("{:?}", hm);

    // panic!("crush and burn");
    // csv reader https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html
    if File::exists("/tmp/urls-iterget.csv") {
        let s = File::read("/tmp/urls-iterget.csv");
        let mut _reader = csv::Reader::from_reader(s.as_bytes());
    }
    // for (i, r) in reader.records().enumerate() {
    //     if i == 0 {
    //         let r = r?;
    //         println!("{:?}", &r);
    //         println!("{}", &r[2]);
    //         let items:Vec<_> = r[2].split("/").collect();
    //         println!("{}", &items.last().unwrap().split(".").collect::<Vec<_>>().first().unwrap());
    //     }
    // }
    // 2021-07-07 Line 86
    let mut rng = rand::thread_rng();
    println!("Random u32 {}", rng.gen::<u32>());
    println!("Random i32 {}", rng.gen::<i32>());
    println!("Random range {}", rng.gen_range(0..10));

    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    println!("{:?}", &vec);

    let matches = App::new("rf: RustFast ")
        .version("v0.1.0")
        .author("GaoangLau")
        .about("Rust Fast script assembler.")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("A cool file"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .takes_value(true)
                .help("Your favorite number"),
        )
        .get_matches();

    let _file = matches.value_of("file").unwrap_or("README.md");
    println!("input file is {}", &_file);
    let num_str = matches.value_of("number");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!("Your favorite number is: {}", n),
            Err(_) => println!("That is NOT a number: {}", s),
        },
    };
    // 2021-07-08 Line 119
    // let res = reqwest::blocking::get("http://aliyun.com")?;
    // println!("{:?}", res.headers());
    // let body = res.text()?;

    // let ans = Command::new("ls").arg("-l").arg("-a").spawn().expect("ls failed");
    // println!("{:?}", ans.stdout);
    say("Keyboard not found, press F1 to continue.");
    // for c in "金木水火土".chars(){lib::put(c);}
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Vec<_>>();
    println!("{:?}", entries);
    // 2021-07-09 Line 155

    let duration = start.elapsed();
    println!("Time elapsed {:#?}", duration);
    Ok(())
}
