#[derive(Clone, Debug)]
struct PartNumber {
    value: u32,
    indecies: Vec<(i32, i32)>,
}

struct Gear {
    cords: (i32, i32),
    parts: Vec<PartNumber>,
}

impl Gear {
    fn get_ratio(&self) -> u32 {
        self.parts[0].value * self.parts[1].value
    }
}

fn main() {
    const INPUT: &str = include_str!("./input");

    let line_length: usize = INPUT.lines().next().unwrap().len();
    let line_amount: usize = INPUT.lines().count();

    let mut grid = vec![vec!['.'; line_amount]; line_length];

    let mut possible_parts: Vec<PartNumber> = vec![];
    let mut possible_gears: Vec<(i32, i32)> = vec![];

    INPUT.lines().enumerate().for_each(|(y, line)| {
        let mut current_indecies: Vec<(i32, i32)> = vec![];

        line.chars().enumerate().for_each(|(x, c)| {
            grid[x][y] = c;
            
            if c == '*' {
                possible_gears.push((x as i32, y as i32));
            }

            if c.is_digit(10) {
                current_indecies.push((x as i32, y as i32));
            } else {
                if current_indecies.len() >= 1 {
                    let value = current_indecies
                        .clone()
                        .into_iter()
                        .map(|(x, y)| grid[x as usize][y as usize])
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                    //print!("{} ", value);
                    possible_parts.push(PartNumber {
                        indecies: current_indecies.clone(),
                        value,
                    });
                }
                current_indecies = vec![];
            }
        });
        if current_indecies.len() >= 1 {
            let value = current_indecies
                .clone()
                .into_iter()
                .map(|(x, y)| grid[x as usize][y as usize])
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            //print!("{} ", value);
            possible_parts.push(PartNumber {
                indecies: current_indecies.clone(),
                value,
            });
        }
        //println!();
    });

    let mut parts: Vec<PartNumber> = vec![];

    for part in possible_parts {
        let mut cords_to_check: Vec<(i32, i32)> = vec![];
        for (x, y) in &part.indecies {
            cords_to_check.push((x + 1, y - 1));
            cords_to_check.push((x + 1, *y));
            cords_to_check.push((x + 1, y + 1));

            cords_to_check.push((*x, y - 1));
            cords_to_check.push((*x, y + 1));
            cords_to_check.push((*x, *y)); // I know this is not needed, I just put it there to
                                           // check if I missed something

            cords_to_check.push((x - 1, y - 1));
            cords_to_check.push((x - 1, *y));
            cords_to_check.push((x - 1, y + 1));
        }

        let filtered_cords = cords_to_check.clone().into_iter().filter(|(x, y)| {
            if x < &0 || y < &0 {
                return false;
            } else if x >= &(line_length as i32) || y >= &(line_amount as i32) {
                return false;
            }
            let c = grid[*x as usize][*y as usize];

            if c.is_digit(10) {
                return false;
            } else if c == '.' {
                return false;
            } else {
                return true;
            }
        });

        if filtered_cords.count() > 0 {
            parts.push(part);
        }
    }

    let mut gears: Vec<Gear> = vec![];

    for (x,y) in possible_gears {
        let part_numbers = parts
            .clone()
            .into_iter()
            .filter(|part| part.indecies.clone().into_iter().filter(|(x1, y1)| (((x - x1).pow(2) + (y - y1).pow(2)) as f64).sqrt() <= (2 as f64).sqrt()).count() > 0)
            .collect::<Vec<PartNumber>>();

        if part_numbers.len() == 2 {
            gears.push(Gear {
                cords: (x, y),
                parts: part_numbers,
            });
        }
    }

    println!("Day 3, Task 2: {}", gears.into_iter().map(|gear| gear.get_ratio()).sum::<u32>());
}
