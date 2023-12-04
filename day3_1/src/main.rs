fn main() {
    const INPUT: &str = include_str!("./input");

    let line_length = INPUT.lines().next().unwrap().len();
    let line_amount = INPUT.lines().count();

    let mut grid = vec![vec!['.'; line_amount]; line_length];
    let mut possible_parts: Vec<PartNumber> = Vec::new();

    INPUT.lines().enumerate().for_each(|(y, line)| {
        let mut current_indecies: Vec<(i32, i32)> = vec![];
        let mut value = 0;

        line.chars().enumerate().for_each(|(x, c)| {
            grid[x][y] = c;

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

    let result: u32 = possible_parts
        .iter()
        .filter(|part| {
            let symbols =
                get_relative_indices(part.indecies[0], part.indecies[part.indecies.len() - 1])
                    .iter()
                    .filter(|(x, y)| {
                        (0..line_length).contains(&(*x as usize))
                            && (0..line_amount).contains(&(*y as usize))
                            && !grid[*x as usize][*y as usize].is_digit(10)
                            && grid[*x as usize][*y as usize] != '.'
                    })
                    .count();
            symbols > 0
        })
        .map(|part| part.value)
        .sum();

    assert_eq!(result, 514969);
    println!("Day 3, Task 1: {}", result);
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

struct PartNumber {
    value: u32,
    indecies: Vec<(i32, i32)>,
}
