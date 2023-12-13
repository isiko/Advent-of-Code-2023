fn main() {
    const INPUT: &str = include_str!("input");

    let maps: Vec<Map> = INPUT
        .lines()
        .fold((Vec::new(), Vec::new()), |mut acc: (Vec<Vec<Vec<char>>>, Vec<Vec<char>>), line: &str| {
            let (acc, current) = &mut acc;
            if line.is_empty() {
                acc.push(current.to_vec());
                current.clear();
            } else {
                current.push(line.chars().collect());
            };
            (acc.to_vec(), current.to_vec())
        })
        .0
        .iter()
        .map(|map| Map::new(map.to_vec()))
        .collect();


    let mut result = 0;
    for map in maps.iter() {
        //println!("map: {:?}", map);

        for line in 0..map.height-1 {
            if map.check_horizontal_reflection(line) {
                result += 100 * (line + 1);
                break;
            }
        }
        for column in 0..map.width-1 {
            //println!("column: {}", column);
            if map.check_vertical_reflection(column) {
                result += column + 1;
                break;
            }
        }
    }

    println!("Day 13 Task 2: {}", result);
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(map: Vec<Vec<char>>) -> Map {
        let height = map.len();
        let width = map[0].len();
        Map { map, width, height }
    }

    fn get_line(&self, line: usize) -> Option<&Vec<char>> {
        if line >= self.height {
            return None;
        }
        Some(&self.map[line])
    }

    fn get_column(&self, column: usize) -> Option<Vec<char>> {
        if column >= self.width {
            return None;
        }
        let mut result = Vec::new();
        for line in self.map.iter() {
            result.push(line[column]);
        }
        Some(result)
    }

    fn check_horizontal_reflection(&self, line: usize) -> bool {
        let mut smudges = 0;
        for x in 0..self.height {
            if x > line || line + x + 1 >= self.height {
                return smudges == 1;
            }
            let upper_line = self.get_line(line - x);
            let lower_line = self.get_line(line + x + 1);
            if upper_line.is_none() || lower_line.is_none() {
                return smudges == 1;
            }
            //println!("upper_line: {}", upper_line.unwrap().iter().collect::<String>());
            //println!("lower_line: {}", lower_line.unwrap().iter().collect::<String>());
            if upper_line != lower_line {
                for (u, l) in upper_line.unwrap().iter().zip(lower_line.unwrap().iter()) {
                    if u != l {
                        smudges += 1;
                    }
                    if smudges > 1 {
                        return false;
                    }
                }
            }
        }
        return false;
    }

    fn check_vertical_reflection(&self, column: usize) -> bool {
        let mut smudges = 0;
        for x in 0..self.width {
            if x > column || column + x + 1 >= self.width {
                return smudges == 1;
            }
            let upper_column = self.get_column(column - x);
            let lower_column = self.get_column(column + x + 1);
            if upper_column.is_none() || lower_column.is_none() {
                return smudges == 1;
            }
            //println!("upper_column: {}", upper_column.unwrap().iter().collect::<String>());
            //println!("lower_column: {}", lower_column.unwrap().iter().collect::<String>());
            if upper_column != lower_column {
                for (u, l) in upper_column.unwrap().iter().zip(lower_column.unwrap().iter()) {
                    if u != l {
                        smudges += 1;
                    }
                    if smudges > 1 {
                        return false;
                    }
                }
            }
        }
        return false;
    }
}

// 30660 (low)
