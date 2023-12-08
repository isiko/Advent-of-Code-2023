use std::collections::HashMap;

use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("input");
    let first = INPUT.lines().next().unwrap().chars().collect::<Vec<char>>();

    let mut start_nodes: Vec<String> = Vec::new();

    let nodes = INPUT
        .lines()
        .skip(2)
        .map(|l| {
            let re = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)");
            let caps = re.unwrap().captures(l).unwrap();

            let name = caps.get(1).unwrap().as_str().to_string();
            let left = caps.get(2).unwrap().as_str().to_string();
            let right = caps.get(3).unwrap().as_str().to_string();

            (name, left, right)
        })
        .fold(HashMap::new(), |mut acc, (n, l, r)| {
            acc.insert(n.clone(), (l, r));
            if n.ends_with("A") {
                start_nodes.push(n.clone());
            }
            acc
        });

    let mut result = 0;
    let mut dif = 0;
    loop {
        let dir = first.get(dif % first.len()).unwrap();
        start_nodes = start_nodes
            .into_iter()
            .map(|mut node| {
                node = match dir {
                    'L' => nodes.get(&node).unwrap().0.clone(),
                    'R' => nodes.get(&node).unwrap().1.clone(),
                    _ => panic!("Unknown direction"),
                }
                .to_string();
                node
            })
            .collect();
        //println!("Nodes: {:?}", start_nodes);
        dif = (dif + 1) % first.len();
        result += 1;

        if start_nodes.iter().all(|n| n.ends_with("Z")) {
            break;
        } else {
        }
    }

    assert_eq!(result, 22357);
    println!("Distance: {}", result);
}
