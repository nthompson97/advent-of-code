use clap::Parser;

// TODO: implement some generic macro for importing all the year/day modules
use aoc::year2024;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    year: u32,

    #[arg(long)]
    day: u32,
}

fn main() {
    let args = Args::parse();

    println!("Running advent of code {} day {}", args.year, args.day);

    match (args.year, args.day) {
        (2024, 1) => year2024::day01::run(),
        _ => panic!("No solution available!"),
    };
}
