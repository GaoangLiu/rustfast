use rustfast::File;
use std::env;

fn main() {
    println!("rust fast it!");
    let f = File::read("Cargo.toml");
    println!("{}", f);

    File::write("Cargo.toml", "test.txt");
    File::remove("test.txt");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    File::say("Hello");
    File::say("Hello say");
    File::vecsay(&args);
}
