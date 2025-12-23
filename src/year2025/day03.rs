use std::fs;
use std::path::Path;

fn parse(contents: &str) -> Vec<&str> {
    return contents.lines().filter(|x| !x.is_empty()).collect();
}

fn getmax(x: Vec<u32>) -> (usize, u32) {
    let mut index: usize = 0;
    let mut max_value: u32 = 0;

    for (i, value) in x.iter().enumerate() {
        if *value > max_value {
            index = i;
            max_value = *value;
        }
    }

    return (index, max_value);
}

fn joltage(batteries: &str, n: usize) -> u64 {
    // When building the result, the next number in the series will always
    // be the maximum value in a window that still allows us to have values
    // remaining for the rest of the joltage values.

    let len: usize = batteries.len();
    let mut result: u64 = 0;
    let mut index: usize = 0;

    println!("{batteries} ({len} values)");

    let batteries: Vec<u32> = batteries.chars().map(|x| x.to_digit(10).unwrap()).collect();

    for i in len - n..len {
        let (max_index, max_value) = getmax(batteries[index..i + 1].to_vec());

        println!(
            "{}..{}, {:?} => {} @ index {}",
            index,
            i,
            &batteries[index..i + 1],
            max_value,
            max_index
        );

        result = 10 * result + (max_value as u64);
        index += 1 + max_index;
    }

    return result;
}

pub fn run() -> () {
    let path = Path::new("/home/ubuntu/repo/inputs/2025/day03.txt");
    let contents = fs::read_to_string(path).unwrap();

    let data = parse(&contents);

    let answer_1: u64 = data.iter().map(|x| joltage(x, 2)).sum();
    println!("Solution to 2025 day 03 part 1: {answer_1}");

    let answer_2: u64 = data.iter().map(|x| joltage(x, 12)).sum();
    println!("Solution to 2025 day 03 part 2: {answer_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joltage() {
        let data: Vec<(&str, usize, u64)> = vec![
            ("987654321111111", 2, 98),
            ("811111111111119", 2, 89),
            ("234234234234278", 2, 78),
            ("818181911112111", 2, 92),
            ("987654321111111", 12, 987654321111),
            ("811111111111119", 12, 811111111119),
            ("234234234234278", 12, 434234234278),
            ("818181911112111", 12, 888911112111),
        ];

        for (batteries, n, expected) in data {
            assert_eq!(joltage(batteries, n), expected);
        }
    }
}
