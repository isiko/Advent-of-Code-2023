use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("./input");
    let seeds = INPUT
        .lines()
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap();

    let split = seeds.split(" ");

    let split1 = split.clone().step_by(2);
    let split2 = split.clone().skip(1).step_by(2);
    
    let seeds: HashSet<u64> = split1
        .zip(split2)
        .map(|(start, len)| (start.parse::<u64>().unwrap(), len.parse::<u64>().unwrap()))
        .map(|(start, len)| {start..start + len})
        .fold(HashSet::new(), |mut acc, range| {
            println!("{:?}", range);
            acc.extend(range);
            acc
        });

    println!("Seed Loading Done");

    let mut maps: Vec<Map> = Vec::new();
    let mut ranges: Vec<Range> = Vec::new();

    for line in INPUT.lines().skip(2) {
        if line.is_empty() {
            maps.push(Map { ranges });
            ranges = Vec::new();
            continue;
        }

        let split = line.split(" ");

        if split.clone().into_iter().count() != 3 {
            continue;
        }

        let split: Vec<u64> = split.map(|x| x.parse::<u64>().unwrap()).collect();

        ranges.push(Range::new(split[1], split[0], split[2]));
    }

    println!("Map Loading Done");

    let result = seeds
        .iter()
        .map(|x| {
            let mut result = *x;
            for map in &maps {
                result = map.apply(result);
            }
            result
        })
        .min()
        .unwrap();

    // assert_eq!(result, 514969);
    println!("Day 5, Task 1: {}", result);
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn apply(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if range.is_in_range(&value) {
                return range.apply(value);
            }
        }
        value
    }
}

#[derive(Debug)]
struct Range {
    diff: i64,
    input: (u64, u64)
}

impl Range {
    fn new(from_start: u64, to_start: u64, length: u64) -> Self {
        Self {
            diff: (to_start as i64) - (from_start as i64),
            input: (from_start, from_start + length)
        }
    }
    fn is_in_range(&self, value: &u64) -> bool {
        value >= &self.input.0 && value <= &self.input.1
    }

    fn apply(&self, value: u64) -> u64 {
        (value as i64 + self.diff) as u64
    }
}
