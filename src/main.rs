use std::env; 
use std::fs;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);


    let contents = fs::read_to_string(filename)
        .expect("read uncorrectly");
    println!("content:\n{}",contents );
}
