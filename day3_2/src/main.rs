use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("./input");

    let line_length = INPUT.lines().next().unwrap().len();
    let line_amount = INPUT.lines().count();

    let mut grid = vec![vec!['.'; line_amount]; line_length];
    let mut possible_parts: Vec<PartNumber> = Vec::new();
    let mut possible_gears: Vec<Gear> = Vec::new();

    INPUT.lines().enumerate().for_each(|(y, line)| {
        let mut current_indecies: Vec<(i32, i32)> = vec![];
        let mut value = 0;

        line.chars().enumerate().for_each(|(x, c)| {
            grid[x][y] = c;

            if c == '*' {
                possible_gears.push(Gear {
                    cords: (x as i32, y as i32),
                    parts: vec![],
                });
            }

            if c.is_digit(10) {
                current_indecies.push((x as i32, y as i32));
                value = value * 10 + c.to_digit(10).unwrap();
            }

            if !c.is_digit(10) || x == line_length - 1 {
                if current_indecies.len() >= 1 {
                    possible_parts.push(PartNumber {
                        indecies: current_indecies.clone(),
                        value,
                    });
                }
                current_indecies = vec![];
                value = 0;
            }
        });
    });

    let parts_hash: HashMap<(i32, i32), PartNumber> = possible_parts
        .into_iter()
        .filter(|part| {
            get_relative_indices(part.indecies[0], part.indecies[part.indecies.len() - 1])
                .iter()
                .filter(|(x, y)| {
                    (0..line_length).contains(&(*x as usize))
                        && (0..line_amount).contains(&(*y as usize))
                        && !grid[*x as usize][*y as usize].is_digit(10)
                        && grid[*x as usize][*y as usize] != '.'
                })
                .count()
                > 0
        })
        .collect::<Vec<PartNumber>>()
        .iter()
        .map(|part| part.indecies.iter().map(move |cord| (cord, part)))
        .flatten()
        .fold(HashMap::new(), |mut acc, (cord, part)| {
            acc.insert(*cord, part.clone());
            acc
        });

    let result = possible_gears
        .into_iter()
        .map(|mut gear| {
            let check_cords = get_relative_indices(gear.cords, gear.cords);
            gear.parts = check_cords
                .iter()
                .flat_map(|cord| parts_hash.get(&cord))
                .cloned()
                .collect();
            gear.parts.sort_by_key(|part| part.value);
            gear.parts.dedup();
            gear
        })
        .map(|gear| {
            //println!("{:?}", gear);
            gear
        })
        .filter(|gear| gear.parts.len() == 2)
        .map(|gear| gear.parts[0].value * gear.parts[1].value)
        .sum::<u32>();

    assert_eq!(result, 78915902);
    println!("Day 3, Task 2: {}", result);
}

fn get_relative_indices(start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
    let y = start.1;
    let mut result = vec![];
    for x in (start.0 - 1)..(end.0 + 2) {
        result.push((x, y + 1));
        result.push((x, y - 1));
    }
    result.push((start.0 - 1, y));
    result.push((end.0 + 1, y));
    result
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PartNumber {
    value: u32,
    indecies: Vec<(i32, i32)>,
}

impl PartialEq<u32> for PartNumber {
    fn eq(&self, other: &u32) -> bool {
        self.value == *other
    }
}

#[derive(Clone, Debug)]
struct Gear {
    cords: (i32, i32),
    parts: Vec<PartNumber>,
}
