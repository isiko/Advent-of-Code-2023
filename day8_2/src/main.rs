use std::collections::HashMap;

use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("input");
    let first = INPUT.lines().next().unwrap().chars().collect::<Vec<char>>();
    let offset: usize = first.len();
    let re = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)");

    let mut start_vertecies: Vec<(String, (String, String))> = Vec::new();

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
            if n.ends_with("A") {
                start_vertecies.push((n.clone(), (l.clone(), r.clone())));
            }
            acc.insert(n.clone(), (l, r));
            acc
        });

    let result = start_vertecies.into_iter().map(|(mut current, _)| {
        let mut result = 0;
        while !current.ends_with("Z") {
            let dir = first.get(result % offset).unwrap();

            current = match dir {
                'L' => nodes.get(&current).unwrap().0.clone(),
                'R' => nodes.get(&current).unwrap().1.clone(),
                _ => panic!("Unknown direction"),
            }
            .to_string();
            result += 1;
        }
        result
    }).fold(1, |a, b| (a / gcd(a,b)) * b );

    assert_eq!(result, 10371555451871);
    println!("Distance: {}", result);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
