use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::string::String;

// hold the x,y position
// ? #[readonly::make]
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
		assert!(self.still_inside(Position{x: pos.x, y: pos.y}));
		
        let line = &self.map[pos.y];
        let line_len = line.chars().count();
        line.chars().nth(pos.x % line_len).unwrap() == String::from("#").chars().nth(0).unwrap()
    }
    
    fn still_inside(&self, pos: Position) -> bool {
		pos.y < self.map.len()
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
    
    // ....#......#..  ->  tree (0,11)
    #[test]
    fn test_map_extend_to_the_right() {
        let p = Position { x: 8, y: 0 };
        let f = ForestMap {
            map: (vec![String::from("..#...")]),
        };
        assert_eq!(f.is_tree(p), true)
    } 

   #[test]
   fn test_still_inside_returns_true_on_border() {
       let p = Position { x: 20, y: 2 };
       let f = ForestMap {
           map: (vec![String::from("..#..."),
                      String::from("..#..."),
                      String::from("..#...")]),
       };
       assert_eq!(f.still_inside(p), true)
   } 
   
   #[test]
   fn test_still_inside_returns_true_inside_border() {
       let p = Position { x: 20, y: 1 };
       let f = ForestMap {
           map: (vec![String::from("..#..."),
                      String::from("..#..."),
                      String::from("..#...")]),
       };
       assert_eq!(f.still_inside(p), true)
   } 
   
   #[test]
   fn test_still_inside_returns_false_outside_border() {
       let p = Position { x: 20, y: 3 };
       let f = ForestMap {
           map: (vec![String::from("..#..."),
                      String::from("..#..."),
                      String::from("..#...")]),
       };
       assert_eq!(f.still_inside(p), false)
   } 
}

// entrypoint from main.rs
pub fn solve() {
    let lines = read_map("input_day03.txt");
    let forest = ForestMap { map: (lines) };
    let p = Position { x: 0, y: 0 };
    let tree_found = forest.is_tree(p);

    println!("it is a tree at position (0,0) : {}", tree_found);
}

// most important helper function to get data from file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
