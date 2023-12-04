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
        self.own_nums.iter().filter(|n| self.winning_nums.contains(n)).count() as u32
    }
}

fn main() {
    let input = include_str!("./input");

    let cards: Vec<Card> = input.lines().map(|s| Card::from_str(s)).collect();

    let mut card_copies = cards.iter().map(|_| 1).collect::<Vec<u32>>();

    for i in 0..cards.len() {
        let wining_nums = cards[i].get_winning_nums();
        for j in i+1..(1 + i+wining_nums as usize) {
            card_copies[j] += card_copies[i];
        }
    }
    

    let sum: u32 = card_copies.iter().sum();
    println!("Day 4, Task 2: {}", sum);
}
