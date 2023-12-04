use std::collections::HashSet;

struct Card {
    winning_nums: HashSet<u32>,
    own_nums: HashSet<u32>,
}

impl Card {
    pub fn from_str(s: &str) -> Card {
        let sets: Vec<HashSet<u32>> = s
            .replace("  ", " ")
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" | ")
            .map(|s| {
                s.split(" ")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<HashSet<u32>>()
            })
            .collect();

        Card {
            winning_nums: sets[0].clone(),
            own_nums: sets[1].clone(),
        }
    }

    pub fn get_winning_nums(&self) -> u32 {
        self.winning_nums.intersection(&self.own_nums).count() as u32
    }
}

fn main() {
    let input = include_str!("./input");

    let cards: Vec<Card> = input.lines().map(|s| Card::from_str(s)).collect();

    let score = cards
        .iter()
        .map(|c| c.get_winning_nums())
        .filter(|s| *s > 0)
        .map(|s| 2_u32.pow(s - 1))
        .sum::<u32>();

    println!("Day 4, Task 1: {}", score);
    assert_eq!(score, 23235);
}
