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
        Game {
            id,
            throws,
        }
    }

    pub fn max_red(&self) -> u8 {
        let mut max = 0;
        for throw in &self.throws {
            if throw.red > max {
                max = throw.red;
            }
        }
        max
    }

    pub fn max_green(&self) -> u8 {
        let mut max = 0;
        for throw in &self.throws {
            if throw.green > max {
                max = throw.green;
            }
        }
        max
    }

    pub fn max_blue(&self) -> u8 {
        let mut max = 0;
        for throw in &self.throws {
            if throw.blue > max {
                max = throw.blue;
            }
        }
        max
    }
}

struct Throw {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Throw {
    const REGEX: &'static str = r"^(\d+) (red|green|blue)$";

    pub fn parse(input: &str) -> Throw {
        let re = Regex::new(Throw::REGEX).unwrap();

        input.split(", ").into_iter().map(|value| {
            let captures = re.captures(value).unwrap();
            let number = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
            let color = captures.get(2).unwrap().as_str();
            (number, color)
        }).fold(Throw{red: 0, blue: 0, green: 0}, |mut acc, (number, color)| {
            match color {
                "red" => acc.red = number,
                "green" => acc.green = number,
                "blue" => acc.blue = number,
                _ => panic!("Unknown color: {}", color),
            }
            acc
        })
    }
}

fn main() {
    let input = include_str!("./input.1");

    let sum = input.lines().map(|line| {
        let game = Game::parse(line);

        return game;
    }).fold(0, |acc, game| {
        let m_red = game.max_red();
        let m_green = game.max_green();
        let m_blue = game.max_blue();

        let power = m_red as u32 * m_green as u32 * m_blue as u32;
        acc + power
    });

    println!("Sum: {}", sum);
}
