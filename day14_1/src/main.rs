fn main() {
    const INPUT: &str = include_str!("input");

    let map = Map::new(INPUT);
    let map = map.tilt_north();

    let result = map.get_load();
    assert_eq!(result, 103614);
    println!("Day 13 Task 1: {}", result);
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        Self { map }
    }

    fn print(&self) {
        for line in self.map.iter() {
            for c in line.iter() {
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    fn get_load(&self) -> usize {
        let mut result = 0;
        for y in 0..self.map.len() {
            let row = &self.map[y];
            for x in 0..row.len() {
                if row[x] == 'O' {
                    result += self.map.len() - y;
                }
            }
        }
        result
    }

    fn turn_right(&self) -> Self {
        let mut map: Vec<Vec<char>> = Vec::new();
        for y in 0..self.map[0].len() {
            let mut row: Vec<char> = Vec::new();
            for x in (0..self.map.len()).rev() {
                row.push(self.map[x][y]);
            }
            map.push(row);
        }
        Self { map }
    }

    fn tilt_north(&self) -> Self {
        let columns: Vec<Vec<char>> = self
            .turn_right()
            .map
            .into_iter()
            .map(|row| row.into_iter().rev().collect::<Vec<char>>())
            .collect();

        let mut final_columns: Vec<Vec<char>> = Vec::new();
        for column in columns {
            let mut ranges: Vec<(usize, usize, bool)> = Vec::new();
            ranges.push((0, 0, false));
            for (i, c) in column.iter().enumerate() {
                match c {
                    'O' => {
                        ranges.last_mut().unwrap().1 += 1;
                    }
                    '#' => {
                        if i == 0 {
                            ranges.last_mut().unwrap().2 = true;
                        } else {
                            ranges.push((i, 0, true));
                        }
                    }
                    _ => {}
                }
            }
            ranges.push((column.len(), 0, false));

            let mut column = Vec::new();

            for range in ranges {
                while column.len() < range.0 {
                    column.push('.');
                }
                if column.len() == range.0 && range.2 {
                    column.push('#');
                }
                for _ in 0..range.1 {
                    column.push('O');
                }
            }
            final_columns.push(column);
        }
        let mut fin_map: Vec<Vec<char>> = Vec::new();

        for i in 0..final_columns[0].len() {
            let mut row = Vec::new();
            for column in final_columns.iter() {
                row.push(column[i]);
            }
            fin_map.push(row);
        }
        Self { map: fin_map }
    }
}
