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
    let part1: (u16, u16) = contents.lines().map(|row| {
        let mut counts = HashMap::new();
        for ch in row.chars() {
            counts.insert(ch, counts.get(&ch).unwrap_or(&0) + 1);
        }
        counts.iter().fold((0, 0), |(doubles, triples), (_, count)| match count {
            2 => (1, triples),
            3 => (doubles, 1),
            _ => (doubles, triples)
        })
    }).fold((0, 0), |(total_doubles, total_triples), (doubles, triples)| (total_doubles + doubles, total_triples + triples)) ;
    println!("{}", part1.0 * part1.1);

    //part 2
    let part2 = contents.lines().filter_map(|row1| contents.lines().map(|row2| row1.chars().zip(row2.chars()).filter_map(|(ch1, ch2)| if ch1 == ch2 { Some(ch1) } else { None }).collect::<String>()).filter(|val| val.len() == 25).next()).next().expect("No id match found");
    println!("{}", part2);
}
