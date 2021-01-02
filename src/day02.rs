use std::io;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::string::String;

pub fn solve() {
        // read file line by line
        let mut counter = 1;
        if let Ok(lines) = read_lines("input_day02.txt") {
            for line in lines {
                
                if let Ok(entry) = line {
                    let entry2 = entry;
                    println!("{}: {}", counter, valid(entry2));
                    counter = counter + 1;
                }
            }
        }

	println!("calling solve {}", valid("1-2 a: abcde".to_string()));
}

fn valid(candidate: String) -> bool {
    print!("{}", candidate);
	return true
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
