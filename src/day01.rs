use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

pub fn solve() {
    let mut inputs = Vec::new();

    // read file line by line
    if let Ok(lines) = read_lines("input_day01.txt") {
        for line in lines {
            if let Ok(entry) = line {
                let number: u32 = entry.parse().unwrap();
                inputs.push(number)
            }
        }
    }

    println!("we have {} entries", inputs.len());

    for x in 1..inputs.len() {
        let first = inputs[x];
        for y in x..inputs.len() {
            let second = inputs[y];
            if first + second == 2020 {
                println!("{} {} {}", first, second, first * second);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
