struct Card {
    id: u32,
    winning_nums: Vec<u32>,
    own_nums: Vec<u32>,
}

impl Card {
    const REGEX: &'static str = r"^Card +(\d)+: *([\d ]+) \| *([\d ]+)$";
    pub fn from_str(s: &str) -> Card {
        let re = regex::Regex::new(Card::REGEX).unwrap();
        let caps = re.captures(s).unwrap();

        let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let winning_nums: Vec<u32> = caps
            .get(2)
            .unwrap()
            .as_str()
            .replace("  ", " ")
            .split(" ")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let own_nums: Vec<u32> = caps
            .get(3)
            .unwrap()
            .as_str()
            .replace("  ", " ")
            .split(" ")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        Card {
            id,
            winning_nums,
            own_nums,
        }
    }

    pub fn get_winning_nums(&self) -> u32 {
        self.own_nums
            .iter()
            .filter(|n| self.winning_nums.contains(n))
            .count() as u32
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
}
