fn main() {
    let input = include_str!("./input");
    let mut sum = 0;
    input.lines().for_each(|line| {
        let fistNum = line.chars().find(|c| c.is_digit(10)).unwrap();
        let lastNum = line.chars().rev().find(|c| c.is_digit(10)).unwrap();

        let num = fistNum.to_digit(10).unwrap() * 10 + lastNum.to_digit(10).unwrap();

        println!("{}", num);
        sum += num;
    });

    println!("= {}", sum);
}
