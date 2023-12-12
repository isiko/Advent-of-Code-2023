use std::collections::BTreeSet;

fn main() {
    const INPUT: &str = include_str!("input");
    let galaxies = get_galaxies(String::from(INPUT));

    let mut result = 0;
    for i1 in 0..galaxies.len() {
        for i2 in i1 + 1..galaxies.len() {
            if i2 <= i1 {
                continue;
            }
            let g1 = galaxies[i1];
            let g2 = galaxies[i2];
            let dist = get_distance(g1, g2);
            result += dist;
        }
    }

    assert_eq!(result, 10292708);
    println!("Day 11, Task 1: {}", result);
}

fn get_distance(a: (u32, u32), b: (u32, u32)) -> u32 {
    let x = (a.0 as i32 - b.0 as i32).abs() as u32;
    let y = (a.1 as i32 - b.1 as i32).abs() as u32;
    x + y
}

fn get_galaxies(input: String) -> Vec<(u32, u32)> {
    let max_x = input.lines().next().unwrap().len() as u32;
    let mut galaxies: Vec<(u32, u32)> = Vec::new();

    let mut empty_lines: BTreeSet<u32> = BTreeSet::new();
    let mut empty_columns: BTreeSet<u32> = BTreeSet::new();

    for x in 0..max_x {
        empty_columns.insert(x as u32);
    }

    for (y, line) in input.lines().enumerate() {
        if !line.contains("#") {
            empty_lines.insert(y as u32);
        }
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                empty_columns.remove(&(x as u32));
                galaxies.push((x as u32, y as u32));
            }
        }
    }

    galaxies.iter().map(|g| {
        let x = g.0 + empty_columns.range(0..g.0).count() as u32;
        let y = g.1 + empty_lines.range(0..g.1).count() as u32;
        (x, y)
    }).collect::<Vec<_>>()
}

#[test]
fn input_to_galaxies() {
    let correct = vec![
        (4, 0),
        (9, 1),
        (0, 2),
        (8, 5),
        (1, 6),
        (12, 7),
        (9, 10),
        (0, 11),
        (5, 11),
    ];

    assert_eq!(get_galaxies(String::from(include_str!("inputT"))), correct);
}

#[test]
fn dist() {
    let g1 = (4, 0);
    let g2 = (9, 1);
    let g3 = (0, 2);
    let g4 = (8, 5);
    let g5 = (1, 6);
    let g6 = (12, 7);
    let g7 = (9, 10);
    let g8 = (0, 11);
    let g9 = (5, 11);

    assert_eq!(get_distance(g1, g7), 15);
    assert_eq!(get_distance(g1, g2), 6);
    assert_eq!(get_distance(g4, g6), 6);
    assert_eq!(get_distance(g3, g6), 17);
    assert_eq!(get_distance(g8, g9), 5);
    assert_eq!(get_distance(g9, g5), 9);

    assert_eq!(get_distance(g5, g9), 9);
    assert_eq!(get_distance(g6, g3), 17);
    assert_eq!(get_distance(g7, g1), 15);
    assert_eq!(get_distance(g9, g8), 5);
}
