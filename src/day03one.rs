use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::string::String;
use std::ops::Add;

// hold the x,y position
#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


#[cfg(test)]
mod tests_position {
    use super::*;

    #[test]
    fn test_positions_can_be_added() {
        let p_1 = Position { x: 2, y: 3 };
        let p_2 = Position { x: 1, y: 0 };
        
        let p_3 = p_1 + p_2;
        assert_eq!(p_3.x, 3);
        assert_eq!(p_3.y, 3);
    }
}

// represents the forest map we navigate through
struct ForestMap {
    map: Vec<String>,
}

impl ForestMap {
    // returns true, when position contains a tree
    fn is_tree(&self, pos: Position) -> bool {
        assert!(self.still_inside(pos));

        let line = &self.map[pos.y];
        let line_len = line.chars().count();
        line.chars().nth(pos.x % line_len).unwrap() == String::from("#").chars().nth(0).unwrap()
    }

    // returns true, when pos is still inside map
    fn still_inside(&self, pos: Position) -> bool {
        pos.y < self.map.len()
    }
}

#[cfg(test)]
mod tests_forest_map {
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
            map: (vec![
                String::from("..#..."),
                String::from("..#..."),
                String::from("..#..."),
            ]),
        };
        assert_eq!(f.still_inside(p), true)
    }

    #[test]
    fn test_still_inside_returns_true_inside_border() {
        let p = Position { x: 20, y: 1 };
        let f = ForestMap {
            map: (vec![
                String::from("..#..."),
                String::from("..#..."),
                String::from("..#..."),
            ]),
        };
        assert_eq!(f.still_inside(p), true)
    }

    #[test]
    fn test_still_inside_returns_false_outside_border() {
        let p = Position { x: 20, y: 3 };
        let f = ForestMap {
            map: (vec![
                String::from("..#..."),
                String::from("..#..."),
                String::from("..#..."),
            ]),
        };
        assert_eq!(f.still_inside(p), false)
    }
}

// Toboggan is our vehicle of choice. It has only one way to go ...
struct Toboggan {
    map: ForestMap,
    velocity: Position,
    tree_count: u32,
    current_pos: Position,
}

impl Toboggan {
    // travel travels the map until end of the forest and returns the trees encoutered
    fn travel(&mut self) -> u32 {
		assert!(self.velocity.y > 0);
		
		while self.map.still_inside(self.current_pos) {
			if self.map.is_tree(self.current_pos) {
               self.tree_count += 1;
            }
			self.slide()
		}
		self.tree_count
    }

    // slide to the next position
    fn slide(&mut self) {
        self.current_pos = self.current_pos + self.velocity;
    }
}

#[cfg(test)]
mod tests_toboggan {
    use super::*;

    #[test]
    fn test_slide_advances_to_next_position() {
        let forest = ForestMap {
            map: (vec![String::from("..#..."), String::from("..#..."), String::from("..#..."), String::from("..#...")]),
        };

        let mut t = Toboggan {
            map: forest,
            velocity: Position { x: 2, y: 1 },
            current_pos: Position { x: 0, y: 0 },
            tree_count: 0,
        };

        t.slide();
        t.slide();
        let expected = Position { x: 4, y: 2 };

        assert_eq!(t.current_pos.x, expected.x);
        assert_eq!(t.current_pos.y, expected.y);
    }
    
    #[test]
    fn test_travel_trees() {
        let forest = ForestMap {
            map: (vec![String::from("..##......."), 
                       String::from("#...#...#.."),
                       String::from(".#....#..#.")]),
        };

        let mut t = Toboggan {
            map: forest,
            velocity: Position { x: 3, y: 1 },
            current_pos: Position { x: 0, y: 0 },
            tree_count: 0,
        };

        assert_eq!(t.travel(), 1)
	}

    #[test]
    fn test_travel_no_trees() {
        let forest = ForestMap {
            map: (vec![String::from("......"), 
                       String::from("..#..."),
                       String::from(".....#")]),
        };

        let mut t = Toboggan {
            map: forest,
            velocity: Position { x: 1, y: 1 },
            current_pos: Position { x: 0, y: 0 },
            tree_count: 0,
        };

        assert_eq!(t.travel(), 0)
	}
	
    #[test]
    fn test_travel_final() {
        let forest = ForestMap {
            map: (vec![String::from("..##......."), 
                       String::from("#...#...#.."),
                       String::from(".#....#..#."),
                       String::from("..#.#...#.#"),
                       String::from(".#...##..#."),
                       String::from("..#.##....."),
                       String::from(".#.#.#....#"),
                       String::from(".#........#"),
                       String::from("#.##...#..."),
                       String::from("#...##....#"),
                       String::from(".#..#...#.#")]),
        };

        let mut t = Toboggan {
            map: forest,
            velocity: Position { x: 3, y: 1 },
            current_pos: Position { x: 0, y: 0 },
            tree_count: 0,
        };

        assert_eq!(t.travel(), 7)
	}
	
}

// entrypoint from main.rs
pub fn solve() {
    let forest = ForestMap {
        map: read_map("input_day03.txt"),
    };

    let mut t = Toboggan {
        map: forest,
        velocity: Position { x: 3, y: 1 },
        current_pos: Position { x: 0, y: 0 },
        tree_count: 0,
    };
    
    println!("{}", t.travel());
}

pub fn solve_two() {
    let forest1 = ForestMap { map: read_map("input_day03.txt") };
    let forest2 = ForestMap { map: read_map("input_day03.txt") };
    let forest3 = ForestMap { map: read_map("input_day03.txt") };
    let forest4 = ForestMap { map: read_map("input_day03.txt") };
    let forest5 = ForestMap { map: read_map("input_day03.txt") };
    
    let mut t1 = Toboggan {
        map: forest1,
        velocity: Position { x: 1, y: 1 },
        current_pos: Position { x: 0, y: 0 },
        tree_count: 0,
    };
    
    let mut t2 = Toboggan {
        map: forest2,
        velocity: Position { x: 3, y: 1 },
        current_pos: Position { x: 0, y: 0 },
        tree_count: 0,
    };
    
    let mut t3 = Toboggan {
        map: forest3,
        velocity: Position { x: 5, y: 1 },
        current_pos: Position { x: 0, y: 0 },
        tree_count: 0,
    };

    let mut t4 = Toboggan {
        map: forest4,
        velocity: Position { x: 7, y: 1 },
        current_pos: Position { x: 0, y: 0 },
        tree_count: 0,
    };

    let mut t5 = Toboggan {
        map: forest5,
        velocity: Position { x: 1, y: 2 },
        current_pos: Position { x: 0, y: 0 },
        tree_count: 0,
    };
    
    println!("{}",   t1.travel() 
                   * t2.travel() 
                   * t3.travel() 
                   * t4.travel() 
                   * t5.travel());
    
    
}

/***************************************
                __        _
              _/  \    _(\(o
             /     \  /  _  ^^^o
            /   !   \/  ! '!!!v'
           !  !  \ _' ( \____
           ! . \ _!\   \===^\)
BEWARE     \ \_!  / __!
THERE WILL  \!   /    \
BE      (\_      _/   _\ )
DRAGONS   \ ^^--^^ __-^ /(__
         ^^----^^    "^--v'

*/
// most important helper function to get data from file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
