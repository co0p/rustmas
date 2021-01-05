use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::string::String;

pub fn solve() {
    // read file line by line
    let mut counter = 0;
    if let Ok(lines) = read_lines("input_day02.txt") {
        for line in lines {
            if let Ok(entry) = line {
                
                if valid_password(extract(entry)) {
                    counter += 1;
                }
            }
        }
    }
    println!("found {counter} valid pwds", counter = counter);
}

fn get_string_from_option(opt: Option<&str>) -> String {
    String::from(opt.unwrap())
}


struct Candidate {
    first: usize,
    second: usize,
    c: String,
    pwd: String,
}


fn valid_password(candidate : Candidate) -> bool {

    let char_first = candidate.pwd.chars().nth(candidate.first-1).unwrap();
    let char_second = candidate.pwd.chars().nth(candidate.second-1).unwrap();
    let c = candidate.c.chars().nth(0).unwrap();

    if char_first == c && char_second != c {
        return true;
    }
    if char_first != c && char_second == c {
        return true;
    }

    false
}


fn extract(line: String) -> Candidate {
    let mut tokens = line.split_whitespace();
    let mut range = tokens.next().unwrap().split("-");
    let lower = get_string_from_option(range.next()).parse::<usize>().unwrap();
    let upper = get_string_from_option(range.next()).parse::<usize>().unwrap();
    let mut letter = get_string_from_option(tokens.next());
    letter.pop(); // drop ':'

    let pwd = get_string_from_option(tokens.next());

    Candidate { first: lower, second: upper, c: letter, pwd: pwd }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found() {
        let candidate = extract(String::from("1-4 a: bbbbbbb"));
        let res = valid_password(candidate);
        assert_eq!(res, false)
    }

    #[test]
    fn test_exactly_one_first_position() {
        let candidate = extract(String::from("2-4 a: babbbbbbb"));
        let res = valid_password(candidate);
        assert_eq!(res, true)
    }

    #[test]
    fn test_exactly_one_second_position() {
        let candidate = extract(String::from("2-4 a: bbbabbbbb"));
        let res = valid_password(candidate);
        assert_eq!(res, true)
    }

    #[test]
    fn test_both_match() {
        let candidate = extract(String::from("2-4 a: bababbb"));
        let res = valid_password(candidate);
        assert_eq!(res, false)
    }
}
