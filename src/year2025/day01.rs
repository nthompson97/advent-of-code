use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
}

fn parse(contents: &str) -> Vec<(Direction, i32)> {
    let lines: Vec<&str> = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let mut data: Vec<(Direction, i32)> = Vec::new();

    for line in lines {
        let (dir, magnitude) = line.split_at(1);

        let dir_enum = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction: {dir}"),
        };
        let magnitude: i32 = magnitude.parse().expect("Expecting a numerical value");

        data.push((dir_enum, magnitude));
    }

    data
}

fn part1(data: &Vec<(Direction, i32)>) -> i32 {
    let mut count: i32 = 0;
    let mut position: i32 = 50;

    for (direction, magnitude) in data {
        match direction {
            Direction::Left => position -= magnitude,
            Direction::Right => position += magnitude,
        }

        while position < 0 {
            position += 100;
        }

        position = position % 100;

        if position == 0 {
            count += 1;
        }

        println!("Moved {direction:?} {magnitude} steps, count is {count} position is {position}");
    }

    count
}

fn part2(data: &Vec<(Direction, i32)>) -> i32 {
    1
}

pub fn run() -> () {
    let path = Path::new("/home/ubuntu/repo/inputs/2025/day01.txt");
    let contents = fs::read_to_string(path).unwrap();

    let data = parse(&contents);

    let answer_1 = part1(&data);
    println!("2024 day 1, part 1 answer: {}", answer_1);

    let answer_2 = part2(&data);
    println!("2024 day 1, part 2 answer: {}", answer_2);
}

#[cfg(test)]
mod tests {

    use super::*;
    const CONTENTS: &str = "
        L99
        R14
        L82";

    #[test]
    fn test_parse() {
        let data = parse(&CONTENTS);
        let expected = vec![
            (Direction::Left, 99),
            (Direction::Right, 14),
            (Direction::Left, 82),
        ];

        assert_eq!(data, expected);
    }

    #[test]
    fn test_part1() {
        let contents: &str = "
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82";
        let data = parse(&contents);
        let answer = part1(&data);

        assert_eq!(answer, 3);
    }
}
