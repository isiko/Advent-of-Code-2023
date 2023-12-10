use std::collections::{HashMap, HashSet};

fn main() {
    const INPUT: &str = include_str!("input");
    let map: Map = Map::new(INPUT);

    //map.print();

    let mut distance_map: Vec<(Coordinate, u32)> =
        map.get_distance_to_start().iter().cloned().collect();
    distance_map.sort_by(|a, b| b.1.cmp(&a.1));
    distance_map.reverse();

    let result = distance_map.last().unwrap().1;

    //for coordinate in map.clone().map.keys() {
    //    let pipe = map.get_pipe(coordinate);
    //    if pipe.is_none() {
    //        continue;
    //    }
    //    let pipe = pipe.unwrap();
    //    for direction in pipe.get_connected_directions() {
    //        let moved_cord = coordinate.move_to(&direction, &max_x, &max_y);
    //        let moved_pipe = map.map.get(&moved_cord).unwrap();

    //        if !moved_pipe.is_open(direction.turn_180()) {
    //            println!("{}/{}", coordinate.x, coordinate.y);
    //            map.remove_all_connected_pipes(&coordinate);
    //            break;
    //        }
    //    }
    //}

    println!("Day 9, Task 1: {}", result);
}

#[derive(Clone)]
struct Map {
    map: HashMap<Coordinate, Pipe>,
    max_x: u32,
    max_y: u32,
    start: Coordinate,
}

impl Map {
    fn new(input: &str) -> Map {
        let max_y = input.lines().count() as u32;
        let max_x = input.lines().next().unwrap().chars().count() as u32;
        let mut start = Coordinate::new(0, 0);

        let mut map: HashMap<Coordinate, Pipe> = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pipe = Pipe::from_char(c);
                if pipe == Pipe::Start {
                    start = Coordinate::new(x as u32, y as u32);
                }
                map.insert(Coordinate::new(x as u32, y as u32), pipe);
            }
        }

        Map {
            map,
            max_x,
            max_y,
            start,
        }
    }

    // fn print(&self) {
    //     for y in 0..self.max_y {
    //         for x in 0..self.max_x {
    //             let pipe = self.get_pipe(&Coordinate::new(x, y));
    //             match pipe {
    //                 Pipe::Start => print!("S"),
    //                 Pipe::CornerNE => print!("L"),
    //                 Pipe::CornerES => print!("F"),
    //                 Pipe::CornerSW => print!("7"),
    //                 Pipe::CornerWN => print!("J"),
    //                 Pipe::Vertical => print!("|"),
    //                 Pipe::Horizontal => print!("-"),
    //                 Pipe::None => print!("."),
    //             }
    //         }
    //         println!();
    //     }
    // }

    fn get_pipe(&self, coordinate: &Coordinate) -> &Pipe {
        self.map.get(coordinate).unwrap()
    }

    fn get_distance_to_start(&self) -> HashSet<(Coordinate, u32)> {
        let mut known_coordinates: HashSet<Coordinate> = HashSet::new();
        let mut result = HashSet::new();
        let mut queue = vec![(self.start.clone(), 0)];

        while !queue.is_empty() {
            queue.sort_by(|a, b| b.1.cmp(&a.1));
            let (current, distance) = queue.pop().unwrap();
            if known_coordinates.contains(&current) {
                continue;
            }
            known_coordinates.insert(current.clone());

            result.insert((current.clone(), distance));
            let pipe = self.get_pipe(&current);

            //println!("{}/{}: ({}) {:?}", current.x, current.y, distance, pipe);
            for direction in pipe.get_connected_directions() {
                let moved_cord = current.move_to(&direction, &self.max_x, &self.max_y);
                if self.map.get(&moved_cord).unwrap().is_open(direction.turn_180()) {
                    queue.push((moved_cord, distance + 1));
                }
            }
        }

        result
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Pipe {
    CornerNE,
    CornerES,
    CornerSW,
    CornerWN,
    Vertical,
    Horizontal,
    None,
    Start,
}

impl Pipe {
    fn from_char(input: char) -> Pipe {
        match input {
            'S' => Pipe::Start,
            'L' => Pipe::CornerNE,
            'F' => Pipe::CornerES,
            '7' => Pipe::CornerSW,
            'J' => Pipe::CornerWN,
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            '.' => Pipe::None,
            _ => panic!("Invalid input '{}'", input),
        }
    }

    fn is_open(&self, direction: Direction) -> bool {
        match self {
            Pipe::CornerNE => match direction {
                Direction::North => true,
                Direction::East => true,
                Direction::South => false,
                Direction::West => false,
            },
            Pipe::CornerES => match direction {
                Direction::North => false,
                Direction::East => true,
                Direction::South => true,
                Direction::West => false,
            },
            Pipe::CornerSW => match direction {
                Direction::North => false,
                Direction::East => false,
                Direction::South => true,
                Direction::West => true,
            },
            Pipe::CornerWN => match direction {
                Direction::North => true,
                Direction::East => false,
                Direction::South => false,
                Direction::West => true,
            },
            Pipe::Vertical => match direction {
                Direction::North => true,
                Direction::East => false,
                Direction::South => true,
                Direction::West => false,
            },
            Pipe::Horizontal => match direction {
                Direction::North => false,
                Direction::East => true,
                Direction::South => false,
                Direction::West => true,
            },
            Pipe::Start => match direction {
                Direction::North => true,
                Direction::East => true,
                Direction::South => true,
                Direction::West => true,
            },
            Pipe::None => false,
        }
    }

    fn get_connected_directions(&self) -> Vec<Direction> {
        let mut result = Vec::new();
        for direction in vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if self.is_open(direction) {
                result.push(direction);
            }
        }
        result
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_180(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn new(x: u32, y: u32) -> Coordinate {
        Coordinate { x, y }
    }

    fn move_to(&self, direction: &Direction, max_x: &u32, max_y: &u32) -> Coordinate {
        let mut new_x = self.x as i64;
        let mut new_y = self.y as i64;

        match direction {
            Direction::North => new_y -= 1,
            Direction::East => new_x += 1,
            Direction::South => new_y += 1,
            Direction::West => new_x -= 1,
        }

        if new_x < 0 || new_y < 0 {
            return self.clone();
        }

        if new_x > *max_x as i64 || new_y > *max_y as i64 {
            return self.clone();
        }

        Self::new(new_x as u32, new_y as u32)
    }
}
