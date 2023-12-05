fn main() {
    const INPUT: &str = include_str!("./input");
    let seeds: Vec<u64> = INPUT
        .lines()
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();


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

    println!("Day 5, Task 1: {}", result);
    assert_eq!(result, 806029445);
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
