use std::fs;

use regex::Regex;

fn get_input(file: &str) -> String {
    fs::read_to_string(file).expect("Failed to read input file")
}

fn run(input: &str) -> u32 {
    let regex = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();
    regex
        .captures_iter(input)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
        })
        .sum()
}

fn main() {
    let input = get_input("input/day03.txt");
    let result = run(&input);
    println!("{}", result);
}
