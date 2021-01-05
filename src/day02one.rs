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
                if valid(entry) {
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

fn valid(candidate: String) -> bool {
    // split into range, char and pwd to check
    let mut tokens = candidate.split_whitespace();
    //let range  = get_string_from_option(tokens.next()).split("-");
    let mut range = tokens.next().unwrap().split("-");
    let lower = get_string_from_option(range.next()).parse::<i32>().unwrap();
    let upper = get_string_from_option(range.next()).parse::<i32>().unwrap();
    let mut letter = get_string_from_option(tokens.next());
    letter.pop(); // drop ':'

    let pwd = get_string_from_option(tokens.next());

    let mut char_count = 0;
    for c in pwd.chars() {
        let s = String::from(c);
        if letter == s {
            char_count += 1;
        }
    }

    char_count >= lower && char_count <= upper
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
    fn test_validate_count_below() {
        let candidate = String::from("2-4 a: bbbabbb");
        let res = valid(candidate);
        assert_eq!(res, false)
    }

    #[test]
    fn test_validate_count_lower_bound() {
        let candidate = String::from("2-4 a: bbbaabbb");
        let res = valid(candidate);
        assert_eq!(res, true)
    }

    #[test]
    fn test_validate_count_inside() {
        let candidate = String::from("2-4 a: bbbaaabbb");
        let res = valid(candidate);
        assert_eq!(res, true)
    }

    #[test]
    fn test_validate_count_upper_bound() {
        let candidate = String::from("2-4 a: bbbaaaabbb");
        let res = valid(candidate);
        assert_eq!(res, true)
    }

    #[test]
    fn test_validate_count_above() {
        let candidate = String::from("2-4 a: bbbaaaaaaabbb");
        let res = valid(candidate);
        assert_eq!(res, false)
    }
}
