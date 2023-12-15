fn main() {
    const INPUT: &str = include_str!("input");

    let result = INPUT
        .strip_suffix("\r\n")
        .or(INPUT.strip_suffix("\n"))
        .unwrap_or(INPUT)
        .split(",")
        .map(|s| {
            let hash = hash(&s.to_string());
            //println!("'{}': {}", s, hash);
            hash
        })
        .sum::<u32>();

    println!("Day 13 Task 1: {}", result);
}

fn hash(s: &String) -> u32 {
    let mut current = 0;
    for c in s.chars() {
        let ascii = c as u32;
        current += ascii;
        current *= 17;
        current %= 256;
    }
    current
}

#[test]
fn test_hash() {
    assert_eq!(hash(&"HASH".to_string()), 52);
}
