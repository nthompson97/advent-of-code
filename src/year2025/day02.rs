use std::fs;
use std::path::Path;

fn parse(contents: &str) -> Vec<(u32, u32)> {
    let mut data: Vec<(u32, u32)> = Vec::new();

    for range in contents.split(",") {
        let mut components = range.split("-");

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

fn part1(data: &Vec<(u32, u32)>) -> u32 {
    1
}

fn part2(data: &Vec<(u32, u32)>) -> u32 {
    1
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
    fn test_part1() {
        let data = parse(&CONTENTS);
        let answer = part1(&data);

        assert_eq!(answer, 1227775554);
    }

    #[test]
    fn test_part2() {
        let data = parse(&CONTENTS);
        let answer = part2(&data);

        assert_eq!(answer, 6);
    }
}
