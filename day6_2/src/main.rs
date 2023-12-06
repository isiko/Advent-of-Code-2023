use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("./input");

    let var_name = Regex::new(r" +");
    let les = var_name
        .unwrap()
        .replace_all(INPUT, "")
        .trim()
        .to_string()
        .lines()
        .map(|l| l.split(':').skip(1).next().unwrap().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let result = Race::new(les[0], les[1]).get_ways_to_win();
    assert_eq!(result, 33149631);
    println!("Day 6, Task 1: {}", result)
}

struct Race {
    duration: u64,
    distance: u64,
}

impl Race {
    fn new(duration: u64, distance: u64) -> Self {
        Self { duration, distance }
    }

    fn get_ways_to_win(&self) -> u64 {
        let range = self.get_button_time_range();
        range.1 - range.0 + 1
    }

    fn get_button_time_range(&self) -> (u64, u64) {
        let mut min: u64 = u64::MAX;
        let mut max: u64 = self.duration;
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
