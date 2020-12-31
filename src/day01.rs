use std::fs::File as fs;

pub fn solve() {
 
    // read file line by line
    let input = fs::open("input_day01.txt");
    
    for line in input.iter() {
        println!("{}", line);
    }
}