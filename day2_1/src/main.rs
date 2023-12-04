use regex::Regex;

struct Game {
    pub id: u32,
    pub throws: Vec<Throw>,
}

impl Game {
    const REGEX: &'static str = r"^Game (\d+): ((((\d+ [^,;]+, )*(\d+ [^,;]+))(; )?)*)$";

    pub fn parse(input: &str) -> Game {
        let re = Regex::new(Game::REGEX).unwrap();
        let captures = re.captures(input).unwrap();
        let id = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let throws: Vec<Throw> = captures
            .get(2)
            .unwrap()
            .as_str()
            .split("; ")
            .map(|throw| Throw::parse(throw))
            .collect();
        Game { id, throws }
    }

    pub fn max_rgb(&self) -> (u8, u8, u8) {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for throw in &self.throws {
            if throw.red > max_red {
                max_red = throw.red;
            }
            if throw.green > max_green {
                max_green = throw.green;
            }
            if throw.blue > max_blue {
                max_blue = throw.blue;
            }
        }
        (max_red, max_green, max_blue)
    }
}

struct Throw {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Throw {
    pub fn parse(input: &str) -> Throw {
        input
            .split(", ")
            .into_iter()
            .map(|value| {
                let split = value.split(" ").collect::<Vec<&str>>();
                let number = split[0].parse::<u8>().unwrap();
                let color = split[1];
                (number, color)
            })
            .fold(
                Throw {
                    red: 0,
                    blue: 0,
                    green: 0,
                },
                |mut acc, (number, color)| {
                    match color {
                        "red" => acc.red = number,
                        "green" => acc.green = number,
                        "blue" => acc.blue = number,
                        _ => panic!("Unknown color: {}", color),
                    }
                    acc
                },
            )
    }
}

fn main() {
    let input = include_str!("./input");

    let sum = input
        .lines()
        .map(|line| Game::parse(line))
        .filter(|game| {
            let (m_red, m_green, m_blue) = game.max_rgb();

            return m_red <= 12 && m_green <= 13 && m_blue <= 14;
        })
        .map(|game| game.id)
        .sum::<u32>();

    println!("Day 2, Task 1: {}", sum);
}
