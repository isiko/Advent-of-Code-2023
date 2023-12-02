fn main() {
    let input = include_str!("./input");
    let mut sum = 0;
    input.lines().for_each(|line| {
        let first = to_value(&get_first_num(line).1);
        let last = to_value(&get_last_num(line).1);

        println!("+ {}{}", &first, &last);

        sum += first * 10 + last;
    });
    println!("= {}", sum);
}

fn get_first_num(line: &str) -> (usize, String) {
    let numbers = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    numbers
        .iter()
        .map(|n| line.find(n).map(|r| (r, n.to_string())))
        .flatten()
        .min_by_key(|v| v.0)
        .unwrap()
}

fn get_last_num(line: &str) -> (usize, String) {
    let numbers = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ]
    .iter()
    .map(|n| n.chars().rev().collect::<String>())
    .collect::<Vec<String>>();

    let line = &line.chars().rev().collect::<String>();
    numbers
        .into_iter()
        .flat_map(|n| line.find(&n).map(|r| (r, n.chars().rev().collect::<String>())))
        .min_by_key(|v| v.0)
        .unwrap()
}

fn to_value(reference: &String) -> usize {
    let numbers = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    for i in 0..numbers.len() {
        if numbers[i] == reference {
            return (i) % 9 + 1;
        }
    }
    return 0;
    
}
