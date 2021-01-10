use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::string::String;

// hold the x,y position
struct Position {
    x: usize,
    y: usize,
}

// represents the forest map we navigate through
struct ForestMap {
    map: Vec<String>,
}

impl ForestMap {
    fn is_tree(&self, pos: Position) -> bool {
        let line = &self.map[pos.y];
        return line.chars().nth(pos.x).unwrap() == String::from("#").chars().nth(0).unwrap();
    }
}

fn read_map(path: &str) -> Vec<String> {
    let mut the_map = vec![];

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(entry) = line {
                the_map.push(entry)
            }
        }
    }

    the_map
}

/*
#### map does extend to the right
....#......#..  ->  tree (0,11)

#### stillInside: (0,0) -> true, (0,1000) -> true, (2,0)->true, (3,0) -> false
....#..
....#..
....#..
*/
#[cfg(test)]
mod tests {
    use super::*;

    // ....#.. -> no tree (0,0), tree (0,4)
    #[test]
    fn test_is_tree_finds_tree() {
        let p = Position { x: 2, y: 0 };
        let f = ForestMap {
            map: (vec![String::from("..#...")]),
        };
        assert_eq!(f.is_tree(p), true)
    }

    // ....#.. -> no tree (0,0), tree (0,4)
    #[test]
    fn test_is_tree_finds_no_tree() {
        let p = Position { x: 3, y: 0 };
        let f = ForestMap {
            map: (vec![String::from("..#...")]),
        };
        assert_eq!(f.is_tree(p), false)
    }
}

// entrypoint from main.rs
pub fn solve() {
    let lines = read_map("input_day03.txt");
    let forest = ForestMap { map: (lines) };
    let p = Position { x: 0, y: 0 };
    let treeFound = forest.is_tree(p);

    println!("it is a tree at position (0,0) : {}", treeFound);
}

// most important helper function to get data from file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
