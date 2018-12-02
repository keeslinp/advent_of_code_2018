use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let mut f = File::open(filename).expect("File not Found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file contents");
    let values: Vec<i32> = contents.lines().filter_map(|row| row.parse::<i32>().ok()).collect();
    let part1 = values.iter().fold(0, |acc, val| acc + val);
    println!("{}", part1);

    let mut recorder = HashMap::new();
    recorder.insert(0, ());
    let line_count = values.len();
    let mut curr_val = 0;
    let mut index = 0;
    loop {
        curr_val += values[index % line_count];
        index += 1;
        if recorder.contains_key(&curr_val) {
            break;
        }
        recorder.insert(curr_val, ());
    }
    println!("{}", curr_val);
}
