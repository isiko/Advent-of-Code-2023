use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("./input");

    let var_name = Regex::new(r" +");
    let les = var_name
        .unwrap()
        .replace_all(INPUT, " ")
        .trim()
        .to_string()
        .lines()
        .map(|l| {
            l.split(" ")
                .skip(1)
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let result = les[0]
        .iter()
        .zip(les[1].iter())
        .map(|(dur, dis)| Race::new(*dur, *dis))
        .fold(0, |acc, r| {
            if acc == 0 {
                r.get_ways_to_win()
            } else {
                acc * r.get_ways_to_win()
            }
        });

    assert_eq!(result, 2449062);
    println!("Day 6, Task 1: {}", result)
}

struct Race {
    duration: u32,
    distance: u32,
}

impl Race {
    fn new(duration: u32, distance: u32) -> Self {
        Self { duration, distance }
    }

    fn get_ways_to_win(&self) -> u32 {
        let range = self.get_button_time_range();
        range.1 - range.0 + 1
    }

    fn get_button_time_range(&self) -> (u32, u32) {
        let mut min: u32 = u32::MAX;
        let mut max: u32 = self.duration;
        for hold_time in 0..self.duration {
            let travel_dist = hold_time * (self.duration - hold_time);
            if self.distance < travel_dist {
                min = min.min(hold_time);
                max = hold_time;
            }
        }
        (min, max)
    }
}
