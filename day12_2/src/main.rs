fn main() {
    const INPUT: &str = include_str!("input");
    let mut result = 0;

    for line in INPUT.lines() {
        if line.starts_with("//") {
            continue;
        }
        let mut split = line.split(" ");
        let data = split.next().unwrap();
        let check = split
            .next()
            .unwrap()
            .split(",")
            .map(|g| g.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

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

        let version = parse_line('.', data_actual.to_string(), check_actual.clone(), "".to_string());
        println!("{} => {}", line, version);
        result += version;
    }

    println!("Day 12 Task 1: {}", result);
}

fn parse_line(last_char: char, line: String, mut check: Vec<u32>, mut word: String) -> u32 {
    if line.len() == 0 {
        //println!("{}", word);
        //println!("<============== Found =============>");
        return 1;
    } else {
        let check_sum = check.iter().sum::<u32>();
        let h_sum = line.chars().filter(|c| *c == '#').count() as u32;
        let q_sum = line.chars().filter(|c| *c == '?').count() as u32;
        let char = line.chars().nth(0).unwrap();
        let line: String = line.chars().skip(1).collect();

        //println!("{} {} {} : {:?}", word, char, line, check);

        if check.len() == 0 {
            if h_sum > 0 {
                //println!("No ranges but still Chars");
                return 0;
            }
        } else {
            if check[0] == 0 {
                if last_char == '#' && char == '.' {
                    check = check.into_iter().skip(1).collect();
                } else if last_char == '#' && char == '#' {
                    //println!("New Range not Allowed");
                    return 0;
                }
            } else {
                if last_char == '#' && char == '.' {
                    //println!("Range not done");
                    return 0;
                }
            }
        }

        if !(h_sum..(h_sum + q_sum + 1)).contains(&check_sum) {
            //println!(
            //    "Not enough Chars to Match ({}..{} <=> {})",
            //    h_sum,
            //    h_sum + q_sum + 1,
            //    check_sum
            //);
            return 0;
        }

        match char {
            '.' => {}
            '#' => {
                check[0] -= 1;
            }
            '?' => {
                let mut h_string = "".to_string();
                h_string.push('#');
                h_string.push_str(&line);

                let mut d_string = "".to_string();
                d_string.push('.');
                d_string.push_str(&line);

                return parse_line(last_char, h_string, check.clone(), word.clone())
                    + parse_line(last_char, d_string, check.clone(), word.clone());
            }
            _ => {
                panic!("Unknown Character")
            }
        }
        word.push(char);
        return parse_line(char, line, check, word);
    }
}
//fn parse_line(line: String, mut check: Vec<u32>, last_char: char) -> u32 {
//    if line.len() == 0 {
//        return 1;
//    } else {
//        let check_sum = check.iter().sum::<u32>();
//        let h_sum = line.chars().filter(|c| *c == '#').count() as u32;
//        let q_sum = line.chars().filter(|c| *c == '?').count() as u32;
//
//        if !(h_sum..(h_sum + q_sum + 1)).contains(&check_sum) {
//            return 0;
//        }
//
//        if check.len() == 0 && h_sum + q_sum > 0 {
//            return 0;
//        }
//
//        let char = line.chars().nth(0).unwrap();
//        let line: String = line.chars().skip(1).collect();
//
//        if check[0] == 0 && char == '#' {
//            return 0;
//        }
//
//        match char {
//            '.' => {
//                if check[0] == 0 {
//                    check = check.into_iter().skip(1).collect();
//                }
//            }
//            '#' => {
//                check[0] -= 1;
//            }
//            '?' => {
//                let sum = parse_line(line.clone(), check.clone(), char);
//                if check[0] == 0 {
//                    return sum;
//                }
//                check[0] -= 1;
//                if check[0] == 0 {
//                    check = check.into_iter().skip(1).collect();
//                }
//                return sum + parse_line(line, check, char);
//            }
//            _ => {
//                panic!("Unknown Character")
//            }
//        }
//
//        parse_line(line, check, char)
//    }
//}
