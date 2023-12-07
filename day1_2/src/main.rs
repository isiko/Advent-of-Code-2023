use std::collections::HashMap;

const NUMBERS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("./input");
    let mut sum = 0;

    let mut reverse_map: HashMap<String, i32> = NUMBERS
        .iter()
        .map(|n| n.to_string())
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, n)| {
            map.insert(n.clone(), (i % 9 + 1).try_into().unwrap());
            map.insert(n.chars().rev().collect(), (i % 9 + 1).try_into().unwrap());
            map
        });

    let reverse_nums: Vec<String> = NUMBERS
        .iter()
        .map(|n| n.chars().rev().collect::<String>())
        .collect();

    reverse_nums.iter().enumerate().for_each(|(i, n)| {
        reverse_map.insert(n.clone(), (i % 9 + 1).try_into().unwrap());
    });

    input.lines().for_each(|line| {
        let first = reverse_map.get(&get_first_num(line).1).unwrap();
        let last = reverse_map.get(&get_last_num(line)).unwrap();

        sum += first * 10 + last;
    });
    println!("Day 1, Task 2: {}", sum);
}

fn get_first_num(line: &str) -> (usize, String) {
    NUMBERS
        .iter()
        .map(|n| line.find(n).map(|r| (r, n.to_string())))
        .flatten()
        .min_by_key(|v| v.0)
        .unwrap()
}

fn get_last_num(line: &str) -> String {
    let line = &line.chars().rev().collect::<String>();
    NUMBERS
        .into_iter()
        .flat_map(|n| line.find(&n).map(|r| (r, n)))
        .min_by_key(|v| v.0)
        .map(|(_, n)| n.to_string())
        .unwrap()
}
