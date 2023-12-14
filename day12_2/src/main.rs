// 17788038834112
use memoize::memoize;
use rstest::rstest;

fn main() {
    const INPUT: &str = include_str!("input");
    let mut result = 0;

    for line in INPUT.lines() {
        if line.starts_with("//") {
            continue;
        }

        let (data, check) = parse_line(line.to_string());
        let (data, check) = times_5(data.to_string(), check);
            let version = check_line('.', data.to_string(), check.clone());
            result += version;
            println!("{} => {}", line, version);
    }

    println!("Day 12 Task 1: {}", result);
}

#[test]
fn run_test_input(){
    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    println!("{}", TEST_INPUT);

    let mut result = 0;
    for line in TEST_INPUT.lines() {
        if line.starts_with("//") {
            continue;
        }

        let (data, check) = parse_line(line.to_string());
        let (data, check) = times_5(data.to_string(), check);
        let version = check_line('.', data.to_string(), check.clone());
        result += version;
        println!("{} => {}", line, version);
    }
    assert_eq!(result, 525152);
}

fn times_5 (data: String, check: Vec<usize>) -> (String, Vec<usize>) {
    let mut data5 = data.to_string();
    let mut check5 = check.clone();
    for _ in 0..4 {
        data5.push('?');
        data5.push_str(&data);

        check5.extend(check.clone());
    }
    (data5, check5)
}

fn parse_line (line: String) -> (String, Vec<usize>) {
    let mut split = line.split(" ");
    let data = split.next().unwrap();
    let check = split
        .next()
        .unwrap()
        .split(",")
        .map(|g| g.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (data.to_string(), check)
}

#[memoize]
fn check_line(last_char: char, old_line: String, check: Vec<usize>) -> usize {
    if old_line.len() == 0 {
        return 1;
    } else {
        let mut q_sum = 0;
        let mut h_sum = 0;
        let mut line = "".to_string();
        let mut char = ' ';
        let mut check = check.clone();
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

        if !(check_sum >= h_sum && (h_sum + q_sum) >= check_sum) {
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

                return check_line(last_char, h_string, check.clone())
                    + check_line(last_char, d_string, check);
            }
            _ => {
                panic!("Unknown Character")
            }
        }
        return check_line(char, line, check);
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
    let (data, check) = parse_line(line.to_string());

    let result = check_line('.', data.to_string(), check.clone());
    assert_eq!(result, expected);
}

#[rstest]
#[case((".#", vec![1]), (".#?.#?.#?.#?.#", vec![1,1,1,1,1]))]
#[case(("???.###", vec![1,1,3]), ("???.###????.###????.###????.###????.###", vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3]))]
fn test_input(#[case] data: (&str, Vec<usize>), #[case] expected: (&str, Vec<usize>)) {
    let (data, check) = times_5(data.0.to_string(), data.1);
    let (expected, expected_check) = expected;
    assert_eq!(data, expected);
    assert_eq!(check, expected_check);
}

#[rstest]
#[case("???.### 1,1,3", ("???.###".to_string() , vec![1,1,3]))]
fn test_parse_line(#[case] line: &str, #[case] expected: (String, Vec<usize>)) {
    let result = parse_line(line.to_string());
    let expected = (expected.0.to_string(), expected.1);
    assert_eq!(result, expected);
}

#[rstest]
#[case("???.### 1,1,3", 1)]
#[case(".??..??...?##. 1,1,3", 16384)]
#[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
#[case("????.#...#... 4,1,1", 16)]
#[case("????.######..#####. 1,6,5", 2500)]
#[case("?###???????? 3,2,1", 506250)]
fn test_line_times_5(#[case] line: String, #[case] expected: usize) {
    let (data, check) = parse_line(line.to_string());
    let (data5, check5) = times_5(data, check);

    let result = check_line('.', data5.to_string(), check5.clone());
    assert_eq!(result, expected);
}
