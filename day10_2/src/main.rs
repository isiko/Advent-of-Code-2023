use std::collections::{HashMap, HashSet};

fn main() {
    const INPUT: &str = include_str!("input");
    let map: Map = Map::new(INPUT);

    let result = map.get_inner_fields();

    assert_eq!(result, 525);
    println!("Day 9, Task 1: {}", result);
}

struct Map {
    map: HashMap<Coordinate, Pipe>,
    max_x: u16,
    max_y: u16,
    start: Coordinate,
}

impl Map {
    fn new(input: &str) -> Map {
        let max_y = input.lines().count() as u16;
        let max_x = input.lines().next().unwrap().chars().count() as u16;
        let mut start = Coordinate::new(0, 0);

        let mut map: HashMap<Coordinate, Pipe> = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pipe = Pipe::from_char(c);
                if pipe == Pipe::Start {
                    start = Coordinate::new(x as u16, y as u16);
                }
                map.insert(Coordinate::new(x as u16, y as u16), pipe);
            }
        }

        Map {
            map,
            max_x,
            max_y,
            start,
        }
    }

    fn filter(&mut self) {
        let r#loop = self.get_loop();
        for coordinate in self.map.clone().keys() {
            if !r#loop.contains(coordinate) {
                self.map.insert(coordinate.clone(), Pipe::None);
            }
        }
    }

    fn get_inner_fields(mut self) -> u16 {
        let mut side_1: HashSet<Coordinate> = HashSet::new();

        self.filter();
        let r#loop = self.get_loop();

        for i in 1..r#loop.len() - 2 {
            let current = r#loop.get(i).unwrap();
            let pipe = self.get_pipe(current);

            let last = r#loop.get(i - 1).unwrap();

            match pipe {
                Pipe::Vertical => {
                    let w_cord = current
                        .move_to(&Direction::West, &self.max_x, &self.max_y);
                    let e_cord = current
                        .move_to(&Direction::East, &self.max_x, &self.max_y);

                    if last.y < current.y {
                        if !side_1.contains(&e_cord) {
                            side_1.extend(self.flood_fill(e_cord));
                        }
                    } else if last.y > current.y {
                        if !side_1.contains(&w_cord) {
                            side_1.extend(self.flood_fill(w_cord));
                        }
                    }
                }
                Pipe::Horizontal => {
                    let n_cord = current
                        .move_to(&Direction::North, &self.max_x, &self.max_y);
                    let s_cord = current
                        .move_to(&Direction::South, &self.max_x, &self.max_y);

                    let n_fill = self.flood_fill(n_cord);
                    let s_fill = self.flood_fill(s_cord);

                    if last.x < current.x {
                        side_1.extend(n_fill);
                    } else if last.x > current.x {
                        side_1.extend(s_fill);
                    }
                }
                Pipe::CornerNE | Pipe::CornerES | Pipe::CornerSW | Pipe::CornerWN => {
                    for dir in Pipe::get_closed_directions(pipe) {
                        let moved_cord = current.move_to(&dir, &self.max_x, &self.max_y);
                        if side_1.contains(&moved_cord) {
                            continue;
                        }

                        let fill = self.flood_fill(moved_cord);

                        match pipe {
                            Pipe::CornerNE => {
                                if last.y == current.y {
                                    side_1.extend(fill);
                                } else {
                                }
                            }
                            Pipe::CornerES => {
                                if last.x == current.x {
                                    side_1.extend(fill);
                                } else {
                                }
                            }
                            Pipe::CornerSW => {
                                if last.y == current.y {
                                    side_1.extend(fill);
                                } else {
                                }
                            }
                            Pipe::CornerWN => {
                                if last.x == current.x {
                                    side_1.extend(fill);
                                } else {
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        side_1.len() as u16
    }

    fn flood_fill(&self, cord: Coordinate) -> HashSet<Coordinate> {
        let mut known_cords: HashSet<Coordinate> = HashSet::new();
        let mut queue: Vec<Coordinate> = vec![cord];

        while !queue.is_empty() {
            let coordinate = queue.pop().unwrap();
            let pipe = self.get_pipe(&coordinate);
            if pipe != &Pipe::None {
                continue;
            }

            known_cords.insert(coordinate.clone());
            for direction in vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                let moved_cord = coordinate.move_to(&direction, &self.max_x, &self.max_y);
                if !known_cords.contains(&moved_cord) {
                    queue.push(moved_cord);
                }
            }
        }

        known_cords
    }

    fn get_pipe(&self, coordinate: &Coordinate) -> &Pipe {
        self.map.get(coordinate).unwrap()
    }

    fn get_loop(&self) -> Vec<Coordinate> {
        let mut known_coordinates: HashSet<Coordinate> = HashSet::new();
        let mut result = Vec::new();
        let mut current = self.start.clone();

        loop {
            if current == self.start && !result.is_empty() {
                break;
            }
            known_coordinates.insert(current.clone());
            result.push(current.clone());

            let pipe = self.get_pipe(&current);
            let connected_directions = Pipe::get_connected_directions(pipe);
            current = connected_directions
                .iter()
                .map(|d| (current.move_to(d, &self.max_x, &self.max_y), d))
                .filter(|(c, d)| {
                    c != &current
                        && Pipe::is_open(self.map.get(&c).unwrap(), &d.turn_180())
                        && (!known_coordinates.contains(c) || self.start == *c)
                })
                .map(|(c, _)| c)
                .next()
                .unwrap();
            //for direction in connected_directions {
            //    let moved_cord = current.move_to(&direction, &self.max_x, &self.max_y);
            //    if moved_cord == current {
            //        continue;
            //    }
            //    if Pipe::is_open(self.map.get(&moved_cord).unwrap(), direction.turn_180())
            //        && (!known_coordinates.contains(&moved_cord) || self.start == moved_cord)
            //    {
            //        current = moved_cord;
            //        break;
            //    }
            //}
        }
        result
    }
}

#[derive(Clone, Eq, PartialEq)]
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

    fn is_open(pipe: &Pipe, direction: &Direction) -> bool {
        match pipe {
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

    fn get_closed_directions(pipe: &Pipe) -> Vec<Direction> {
        let mut result = Vec::new();
        for direction in vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if !Pipe::is_open(pipe, &direction) {
                result.push(direction);
            }
        }
        result
    }

    fn get_connected_directions(pipe: &Pipe) -> Vec<Direction> {
        let mut result = Vec::new();
        for direction in vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if Pipe::is_open(pipe, &direction) {
                result.push(direction);
            }
        }
        result
    }
}

#[derive(Clone)]
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
    x: u16,
    y: u16,
}

impl Coordinate {
    fn new(x: u16, y: u16) -> Coordinate {
        Coordinate { x, y }
    }

    fn move_to(&self, direction: &Direction, max_x: &u16, max_y: &u16) -> Coordinate {
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

        if new_x >= *max_x as i64 || new_y >= *max_y as i64 {
            return self.clone();
        }

        Self::new(new_x as u16, new_y as u16)
    }
}
