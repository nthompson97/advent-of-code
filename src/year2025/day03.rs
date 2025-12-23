use std::fs;
use std::path::Path;

fn parse(contents: &str) -> Vec<&str> {
    return contents.lines().filter(|x| !x.is_empty()).collect();
}

fn joltage(batteries: &str) -> u64 {
    println!("{} (there are #{} options)", batteries, batteries.len());

    let mut index_0 = 0;
    let mut value_0 = 0;

    let mut index_1 = 0;
    let mut value_1 = 0;

    for (i, c) in batteries.chars().enumerate() {
        println!("{index_0} {index_1} {i} {c}");

        let val = c.to_digit(10).unwrap() as u64;

        if i < batteries.len() - 1 && val > value_0 {
            value_0 = val;
            index_0 = i;

            // the next value becomes our value 1
            let next_val = batteries.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as u64;
            value_1 = next_val;
            index_1 = i;
        } else if val > value_1 {
            value_1 = val;
            index_1 = i;
        }
    }

    return value_0 * 10 + value_1;
}

pub fn run() -> () {
    let path = Path::new("/home/ubuntu/repo/inputs/2025/day03.txt");
    let contents = fs::read_to_string(path).unwrap();

    let data = parse(&contents);

    let answer_1: u64 = data.iter().map(|x| joltage(x)).sum();
    println!("Solution to 2025 day 03 part 1: {answer_1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joltage() {
        let data: Vec<(&str, u64)> = vec![
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
        ];

        for (batteries, expected) in data {
            assert_eq!(joltage(batteries), expected);
        }
    }
}
