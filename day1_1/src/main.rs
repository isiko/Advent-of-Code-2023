fn main() {
    let input = include_str!("./input");
    let mut sum = 0;
    input.lines().for_each(|line| {
        let fist_num = line.chars().find(|c| c.is_digit(10)).unwrap();
        let last_num = line.chars().rev().find(|c| c.is_digit(10)).unwrap();

        let num = fist_num.to_digit(10).unwrap() * 10 + last_num.to_digit(10).unwrap();

        sum += num;
    });

    println!("Day 1, Task 1: {}", sum);
}
