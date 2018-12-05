use std::env;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

#[derive(Debug)]
struct Square {
    id: u16,
    position: (u16, u16),
    size: (u16, u16),
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let mut f = File::open(filename).expect("File not Found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file contents");
    let mut grid = [[0u32; 2000]; 1000];
    let re = Regex::new(r#"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)"#).expect("regex failed");
    re.captures_iter(&contents).map(|cap| {
        Square {
            id: cap.get(1).unwrap().as_str().parse::<u16>().unwrap(),
            position: (cap.get(2).unwrap().as_str().parse::<u16>().unwrap(), cap.get(3).unwrap().as_str().parse::<u16>().unwrap()),
            size: (cap.get(4).unwrap().as_str().parse::<u16>().unwrap(), cap.get(5).unwrap().as_str().parse::<u16>().unwrap()),
        }
    }).for_each(|Square { position: (xpos, ypos), size: (width,  height), .. }| {
        for x in 0..width {
            for y in 0..height {
                grid[(ypos + y) as usize][(xpos + x) as usize] += 1;
            }
        }
    });
    let count = grid.iter().fold(0, |sum, row| {
        sum + row.iter().fold(0, |agg, val| if *val > 1 { agg + 1 } else { agg })
    });
    println!("{}", count);
}
