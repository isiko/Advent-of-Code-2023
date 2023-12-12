use rstest::rstest;
use std::{sync::mpsc, thread};

fn main() {
    const INPUT: &str = include_str!("input");
    let (tx, rx) = mpsc::channel();

    let mut started_threads = 0;

    for line in INPUT.lines() {
        if line.starts_with("//") {
            continue;
        }
        started_threads += 1;
        let line_tx = tx.clone();
        let mut split = line.split(" ");
        let data = split.next().unwrap();
        let check = split
            .next()
            .unwrap()
            .split(",")
            .map(|g| g.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut data_actual = data.to_string();
        for _ in 0..4 {
            data_actual.push('?');
            data_actual.push_str(data);
        }

        let mut check_actual = check.clone();
        check_actual.extend(check.clone());
        check_actual.extend(check.clone());
        check_actual.extend(check.clone());
        check_actual.extend(check.clone());
        thread::spawn(move || {
            let version = parse_line(
                '.',
                data_actual.to_string(),
                check_actual.clone(),
                &"".to_string(),
            );
            //println!("{} => {}", line, version);
            line_tx.send((line.to_string(), version)).unwrap();
        });
    }
    let mut ended_threads = 0;
    let mut result = 0;
    for recieved in rx {
        let (line, output): (String, usize) = recieved;
        println!("{} => {}", line, output);
        result += output;
        ended_threads += 1;
        println!(
            "Result {} of {}: {}",
            ended_threads, started_threads, result
        );
    }

    println!("Day 12 Task 1: {}", result);
}

fn parse_line(last_char: char, old_line: String, mut check: Vec<usize>, word: &String) -> usize {
    if old_line.len() == 0 {
        //println!("{}", word);
        //println!("<============== Found =============>");
        return 1;
    } else {
        let mut q_sum = 0;
        let mut h_sum = 0;
        let mut line = "".to_string();
        let mut char = ' ';
        for (i, c) in old_line.chars().enumerate() {
            match c {
                '?' => {
                    q_sum += 1;
                }
                '#' => {
                    h_sum += 1;
                }
                _ => {}
            }
            if i == 0 {
                char = c;
            } else {
                line.push(c);
            }
        }

        if check.len() == 0 {
            if h_sum > 0 {
                return 0;
            }
        } else {
            if check[0] == 0 {
                if last_char == '#' && char == '.' {
                    check.remove(0);
                } else if last_char == '#' && char == '#' {
                    return 0;
                }
            } else {
                if last_char == '#' && char == '.' {
                    return 0;
                }
            }
        }

        let check_sum = check.iter().sum::<usize>();

        if !(h_sum..(h_sum + q_sum + 1)).contains(&check_sum) {
            return 0;
        }

        match char {
            '.' => {}
            '#' => {
                check[0] -= 1;
            }
            '?' => {
                let mut h_string = old_line.clone();
                h_string.replace_range(0..1, "#");
                let mut d_string = old_line;
                d_string.replace_range(0..1, ".");

                return parse_line(last_char, h_string, check.clone(), word)
                    + parse_line(last_char, d_string, check, word);
            }
            _ => {
                panic!("Unknown Character")
            }
        }
        let mut word = word.clone();
        word.push(char);
        return parse_line(char, line, check, &word);
    }
}

#[rstest]
#[case("???.### 1,1,3", 1)]
#[case("????.#...#... 4,1,1", 1)]
#[case(".??..??...?##. 1,1,3", 4)]
#[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
#[case("????.######..#####. 1,6,5", 4)]
#[case("?###???????? 3,2,1", 10)]
fn test_line(#[case] line: String, #[case] expected: usize) {
    let mut split = line.split(" ");
    let data = split.next().unwrap();
    let check = split
        .next()
        .unwrap()
        .split(",")
        .map(|g| g.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let result = parse_line('.', data.to_string(), check.clone(), &"".to_string());
    assert_eq!(result, expected);
}
