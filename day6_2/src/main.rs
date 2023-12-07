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

    // Thanks to @tomoshiekah -> https://github.com/tomoshiekah/AoC/blob/75a816801e06a628f1b66bbce1e8c41aca54fc86/AoC2023/day06.py#L34-L52
    fn get_button_time_range(&self) -> (u64, u64) {
        let duration = self.duration as f64;
        let distance = self.distance as f64;

        let mut min = ((-duration + (duration.powf(2.) + 4. * -distance).sqrt()) / -2.).ceil();
        let mut max = ((-duration - (duration.powf(2.) - 4. * distance).sqrt()) / -2.).floor();

        if max * (duration - max) == distance {
            min += 1.;
            max -= 1.;
        }

        (min as u64, max as u64)
    }
}

#[test]
fn test() {
    assert_eq!(Race::new(7, 9).get_button_time_range(), (2, 5));
    assert_eq!(Race::new(15, 40).get_button_time_range(), (4, 11));
    assert_eq!(Race::new(30, 200).get_button_time_range(), (11, 19));
}
