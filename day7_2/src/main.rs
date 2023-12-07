use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("./input");
    let mut hands = INPUT
        .lines()
        .map(|line| Hand::new(line.to_string()))
        .collect::<Vec<Hand>>();
    hands.sort();

    let result = hands
        .iter()
        .enumerate()
        .map(|(i, x)| {
            //println!(
            //    "{}, ('{}', {}, {}), {}",
            //    i + 1,
            //    x.reference.split(" ").next().unwrap(),
            //    match HandType::from_hand(x.clone()) {
            //        HandType::FiveOfAKind => 0,
            //        HandType::FourOfAKind => 1,
            //        HandType::FullHouse => 2,
            //        HandType::ThreeOfAKind => 3,
            //        HandType::TwoPairs => 4,
            //        HandType::OnePair => 5,
            //        HandType::HighCard => 6,
            //    },
            //    x.bid,
            //    x.bid * (i + 1) as u32
            //);
            x.bid * (i as u32 + 1)
        })
        .sum::<u32>();
    assert_eq!(result, 248750699);
    println!("Day 7, Task 2: {}", result)
}

#[derive(Eq, Clone, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    #[allow(dead_code)]
    reference: String,
}

impl Hand {
    fn new(reference: String) -> Self {
        let mut split = reference.split(" ");

        let mut cards = Vec::new();
        let card_string = split.next().unwrap();
        for i in 0..5 {
            let card = card_string.chars().nth(i).unwrap();
            cards.push(Card::new(card));
        }

        let bid = split.next().unwrap().parse::<u32>().unwrap();

        Self {
            cards,
            bid,
            reference,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_hand_type = HandType::from_hand(self.clone());
        let other_hand_type = HandType::from_hand(other.clone());

        if self == other {
            std::cmp::Ordering::Equal
        } else if self_hand_type == other_hand_type {
            let neq_card = self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find(|(s, o)| s.value != o.value)
                .unwrap();
            neq_card.0.value.cmp(&neq_card.1.value)
        } else {
            self_hand_type.cmp(&other_hand_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards
            .iter()
            .zip(other.cards.iter())
            .all(|(s, o)| s.value == o.value)
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl HandType {
    fn value(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 8,
            HandType::FourOfAKind => 7,
            HandType::FullHouse => 6,
            HandType::ThreeOfAKind => 5,
            HandType::TwoPairs => 4,
            HandType::OnePair => 3,
            HandType::HighCard => 2,
        }
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for HandType {}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_value = self.value();
        let other_value = other.value();
        self_value.cmp(&other_value)
    }
}

impl HandType {
    fn from_hand(hand: Hand) -> HandType {
        let cards = hand.cards;
        let jokers = cards.iter().filter(|x| x.reference == 'J').count() as u8;
        let labels = cards.iter().fold(HashMap::new(), |mut acc, card| {
            if card.reference != 'J' {
                let counter = acc.entry(card.value).or_insert(0);
                *counter += 1;
            }
            acc
        });

        let found_label_amount = labels.values().filter(|&x| *x > 0).count();
        let found_label_max = labels.values().max().unwrap_or(&0) + jokers;

        if found_label_max == 5 {
            HandType::FiveOfAKind
        } else if found_label_max == 4 && found_label_amount <= 2 {
            HandType::FourOfAKind
        } else if found_label_max == 3 && found_label_amount <= 2 {
            HandType::FullHouse
        } else if found_label_max == 3 && found_label_amount <= 3 {
            HandType::ThreeOfAKind
        } else if found_label_max == 2 && found_label_amount <= 3 {
            HandType::TwoPairs
        } else if found_label_max == 2 && found_label_amount <= 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Card {
    value: u8,
    reference: char,
}

impl Card {
    fn new(reference: char) -> Self {
        let value = if reference.is_digit(10) {
            reference.to_digit(10).unwrap() as u8
        } else {
            match reference {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                _ => panic!("Invalid card reference"),
            }
        };
        Self { value, reference }
    }
}

#[test]
fn j5() {
    let hand = Hand::new("JJJJJ 100".to_string());
    assert_eq!(HandType::from_hand(hand), HandType::FiveOfAKind);
    let hand = Hand::new("JJJJ2 100".to_string());
    assert_eq!(HandType::from_hand(hand), HandType::FiveOfAKind);
    let hand = Hand::new("JJJ22 100".to_string());
    assert_eq!(HandType::from_hand(hand), HandType::FiveOfAKind);
    let hand = Hand::new("JJ222 100".to_string());
    assert_eq!(HandType::from_hand(hand), HandType::FiveOfAKind);
    let hand = Hand::new("J2222 100".to_string());
    assert_eq!(HandType::from_hand(hand), HandType::FiveOfAKind);
}

#[test]
fn j4() {
    assert_eq!(
        HandType::from_hand(Hand::new("JJJ32 100".to_string())),
        HandType::FourOfAKind
    );
    assert_eq!(
        HandType::from_hand(Hand::new("JJ332 100".to_string())),
        HandType::FourOfAKind
    );
    assert_eq!(
        HandType::from_hand(Hand::new("J3332 100".to_string())),
        HandType::FourOfAKind
    );
}

#[test]
fn j3() {
    assert_eq!(
        HandType::from_hand(Hand::new("JJ432 100".to_string())),
        HandType::ThreeOfAKind
    );
    assert_eq!(
        HandType::from_hand(Hand::new("J4432 100".to_string())),
        HandType::ThreeOfAKind
    );
}
