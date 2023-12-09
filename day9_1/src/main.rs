fn main() {
    const INPUT: &str = include_str!("input");
    let result = INPUT.lines().map(|line| {
        let mut history = Vec::new();
        for num in line.split_whitespace() {
            history.push(num.parse().unwrap());
        }
        history
    }).map(|history| extrapolate_history(history)).sum::<i64>();

    println!("Day 9, Task 1: {}", result);
}

fn extrapolate_history(history: Vec<i64>) -> i64 {
    if history.iter().all(|num| num == &0) {
        return 0;
    }

    let mut abstract_history = Vec::new();
    for (o, a) in history.iter().zip(history.iter().skip(1)) {
        abstract_history.push(a - o);
    }

    history[history.len() - 1] + extrapolate_history(abstract_history)
}
