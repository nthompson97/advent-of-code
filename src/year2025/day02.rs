use std::fs;
use std::path::Path;

fn parse(contents: &str) -> Vec<(u64, u64)> {
    let mut data: Vec<(u64, u64)> = Vec::new();

    for range in contents.split(",") {
        let mut components = range.trim().split("-");

        let start = components
            .next()
            .expect("start")
            .parse()
            .expect("valid number");
        let end = components
            .next()
            .expect("end")
            .parse()
            .expect("valid number");

        data.push((start, end));
    }

    data
}

fn valid_id(s: String) -> bool {
    let len = s.len();

    if len % 2 == 1 {
        return true;
    } else {
        let (a, b) = s.split_at(len / 2);

        return a != b;
    }
}

fn part1(data: &Vec<(u64, u64)>) -> u64 {
    let mut sum: u64 = 0;

    for (start, end) in data {
        for id in *start..=*end {
            let s: String = id.to_string();
            if !valid_id(s) {
                sum += id;
            }
        }
    }

    sum
}

fn valid_id_2(s: String) -> bool {
    // Incrementing one at a time, check if the whole string is equal to
    // repetitions of the same internal string, up to the midpoint.
    let len = s.len();

    for i in 1..1 + (len / 2) {
        let substring = s[..i].repeat(len / i);

        if s == substring {
            return false;
        }
    }

    return true;
}

fn part2(data: &Vec<(u64, u64)>) -> u64 {
    let mut sum: u64 = 0;

    for (start, end) in data {
        for id in *start..=*end {
            let s: String = id.to_string();
            if !valid_id_2(s) {
                sum += id;
            }
        }
    }

    sum
}

pub fn run() -> () {
    let path = Path::new("/home/ubuntu/repo/inputs/2025/day02.txt");
    let contents = fs::read_to_string(path).unwrap();

    let data = parse(&contents);

    let answer_1 = part1(&data);
    println!("2025 day 2, part 1 answer: {}", answer_1);

    let answer_2 = part2(&data);
    println!("2025 day 2, part 2 answer: {}", answer_2);
}

#[cfg(test)]
mod tests {

    use super::*;
    const CONTENTS: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_parse() {
        let data = parse(&CONTENTS);
        let expected = vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124),
        ];

        assert_eq!(data, expected);
    }

    #[test]
    fn test_valid_id() {
        let data: Vec<(u64, bool)> = vec![(1, true), (11, false), (121, true), (1212, false)];
        for (id, validity) in data {
            assert_eq!(valid_id(id.to_string()), validity);
        }
    }

    #[test]
    fn test_part1() {
        let data = parse(&CONTENTS);
        let answer = part1(&data);

        assert_eq!(answer, 1227775554);
    }

    #[test]
    fn test_valid_id_2() {
        let data: Vec<(u64, bool)> = vec![(12341234, false), (123123123, false), (123123124, true)];
        for (id, validity) in data {
            assert_eq!(valid_id_2(id.to_string()), validity);
        }
    }

    #[test]
    fn test_part2() {
        let data = parse(&CONTENTS);
        let answer = part2(&data);

        assert_eq!(answer, 4174379265);
    }
}
