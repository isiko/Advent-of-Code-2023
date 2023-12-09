use std::collections::HashMap;

use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("input");
    let first = INPUT.lines().next().unwrap().chars().collect::<Vec<char>>();
    let offset: usize = first.len();
    let re = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)");

    let nodes = INPUT
        .lines()
        .skip(2)
        .map(|l| {
            let caps = re.clone().unwrap().captures(l).unwrap();

            let name = caps.get(1).unwrap().as_str().to_string();
            let left = caps.get(2).unwrap().as_str().to_string();
            let right = caps.get(3).unwrap().as_str().to_string();

            (name, left, right)
        })
        .fold(HashMap::new(), |mut acc, (n, l, r)| {
            acc.insert(n.clone(), (l, r));
            acc
        });

    let mut result = 0;
    let mut current = "AAA".to_string();
    while current != "ZZZ".to_string() {
        let dir = first.get(result % offset).unwrap();

        current = match dir {
            'L' => nodes.get(&current).unwrap().0.clone(),
            'R' => nodes.get(&current).unwrap().1.clone(),
            _ => panic!("Unknown direction"),
        }
        .to_string();
        result += 1;
    }

    assert_eq!(result, 22357);
    println!("Distance: {}", result);
}
