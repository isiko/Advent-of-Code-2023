fn main() {
    const INPUT: &str = include_str!("./inputT");
    let seeds = INPUT.lines().nth(0).unwrap().split(": ").nth(1).unwrap();

    let split = seeds.split(" ");

    let split1 = split.clone().step_by(2);
    let split2 = split.clone().skip(1).step_by(2);

    let set_handler: SetHandler = split1
        .zip(split2)
        .map(|(start, len)| (start.parse::<u64>().unwrap(), len.parse::<u64>().unwrap()))
        .map(|(start, len)| Set::new(start, len))
        .fold(SetHandler::new(), |acc, set| acc.add(set))
        .print_sets();
    println!("-------------------");

    let mut maps: Vec<Vec<Range>> = Vec::new();
    let mut ranges: Vec<Range> = Vec::new();

    for line in INPUT.lines().skip(2) {
        if line.is_empty() {
            maps.push(ranges);
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
    for m in &maps[5] {
        println!("{:?}", m);
    }

    let set_handlert = maps.iter().fold(set_handler, |acc, map| {
        println!("----- New Rule -----");
        acc.apply_map(map.to_vec())
    });

    let result = set_handlert.get_min();

    //assert_eq!(result, 46);
    println!("Day 5, Task 1: {}", result);
}

#[derive(Debug, Clone)]
struct Range {
    diff: i64,
    set: Set,
}

impl Range {
    fn new(from_start: u64, to_start: u64, length: u64) -> Self {
        Self {
            diff: (to_start as i64) - (from_start as i64),
            set: Set::new(from_start, from_start + length),
        }
    }
}

struct SetHandler {
    sets: Vec<Set>,
}

impl SetHandler {
    fn new() -> Self {
        Self { sets: Vec::new() }
    }

    fn sort(self) -> Self {
        let mut sets = self.sets;
        sets.sort_by(|a, b| a.start.cmp(&b.start));
        Self { sets }
    }

    fn print_sets_msg(self, msg: &str) -> Self {
        println!("{}: {:?}", msg, self.sets);
        self
    }

    fn print_sets(self) -> Self {
        println!("{:?}", self.sets);
        self
    }

    fn add(self, new: Set) -> Self {
        let mut was_merged = false;
        let mut out = SetHandler {
            sets: self
                .sets
                .into_iter()
                .map(|mut set| {
                    if set.mergable(&new) {
                        was_merged = true;
                        set.merge(&new);
                    }
                    set
                })
                .collect(),
        };
        if !was_merged {
            out.sets.push(new);
        }

        out
    }

    fn get_min(&self) -> u64 {
        self.sets.iter().map(|set| set.start).min().unwrap()
    }

    fn get_sets_in_range(&self, range: &Range) -> Vec<Set> {
        self.sets
            .iter()
            .filter(|set| set.mergable(&range.set))
            .map(|set| *set)
            .collect()
    }

    fn move_values(self, range: &Range) -> Self {
        let sets_to_move: Vec<Set> = self.get_sets_in_range(&range).into_iter().collect();

        let new = self.remove(&range.set)
            .print_sets_msg("After Remove");
        let sets_to_move = sets_to_move
            .iter()
            .map(|set| Set {
                start: set.start.max(range.set.start),
                end: set.end.min(range.set.end),
            })
            .collect::<Vec<Set>>();

        println!("Sets to move: {:?}", sets_to_move);

        let sets_to_move: Vec<Set> = sets_to_move
            .into_iter()
            .map(|set| set.move_values(range.diff))
            .collect();

        println!("  After move: {:?}", sets_to_move);

        sets_to_move
            .into_iter()
            .fold(new, |acc, set| acc.add(set))
            .sort()
    }

    fn remove(mut self, other: &Set) -> Self {
        self.sets = self
            .sets
            .into_iter()
            .flat_map(|mut set| set.remove_values(*other))
            .collect();
        self
    }

    fn apply_map(self, map: Vec<Range>) -> Self {
        map.iter().fold(self, |acc, range| {
            acc.move_values(range).print_sets_msg("   After Map")
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Set {
    start: u64,
    end: u64,
}

impl Set {
    fn new(start: u64, length: u64) -> Self {
        Self {
            start,
            end: start + length - 1,
        }
    }

    fn is_in_set(&self, value: &u64) -> bool {
        value >= &self.start && value <= &self.end
    }

    fn mergable(&self, other: &Set) -> bool {
        self.is_in_set(&other.start)
            || self.is_in_set(&other.end)
            || other.is_in_set(&self.start)
            || other.is_in_set(&self.end)
    }

    fn merge(&mut self, other: &Set) {
        if self.mergable(other) {
            self.start = self.start.min(other.start);
            self.end = self.end.max(other.end);
        }
    }

    fn remove_values(&mut self, other: Set) -> Vec<Set> {
        if !self.mergable(&other) {
            return vec![*self];
        }

        let mut result = Vec::new();

        if self.start < other.start {
            result.push(Set {
                start: self.start,
                end: other.start - 1,
            });
        }

        if self.end > other.end {
            result.push(Set {
                start: other.end + 1,
                end: self.end,
            });
        }

        result
    }

    fn move_values(self, diff: i64) -> Set {
        Set {
            start: (self.start as i64 + diff as i64) as u64,
            end: (self.end as i64 + diff as i64) as u64,
        }
    }
}
