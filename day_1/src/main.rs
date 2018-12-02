use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let mut f = File::open(filename).expect("File not Found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file contents");
    let answer = contents.lines().filter_map(|row| row.parse::<i32>().ok()).fold(0, |acc, val| acc + val);
    println!("{}", answer);
}
