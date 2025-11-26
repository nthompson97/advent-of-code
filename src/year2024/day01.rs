use std::fs;
use std::path::Path;

fn parse(contents: &str) -> Vec<(u32, u32)> {
    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a = parts.next().unwrap().parse::<u32>().unwrap();
            let b = parts.next().unwrap().parse::<u32>().unwrap();
            (a, b)
        })
        .collect()
}

fn part1(data: &Vec<(u32, u32)>) -> u32 {
    1
}

fn part2() -> u32 {
    1
}

pub fn run() -> () {
    let path = Path::new("~/repo/inputs/2024/day01.txt");
    let contents = fs::read_to_string(path).unwrap();

    let data = parse(&contents);

    let answer_1 = part1(&data);
    println!("2024 day 1, part 1 answer: {}", answer_1);

    let answer_2 = part2();
    println!("2024 day 1, part 2 answer: {}", answer_2);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let contents = "
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3";
        let data = parse(&contents);
        let expected = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];

        assert_eq!(data, expected);
    }
}
