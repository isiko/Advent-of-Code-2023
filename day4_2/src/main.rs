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

    pub fn get_score(&self) -> u32 {
        let mut score = 0;
        for num in &self.own_nums {
            if self.winning_nums.contains(&num) {
                score = if score == 0 { 1 } else { score * 2 }
            }
        }
        score
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
