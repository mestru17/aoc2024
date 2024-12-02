use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_reports(file: &str) -> Vec<Vec<u32>> {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            line.expect("Failed to read line")
                .split_whitespace()
                .map(|s| s.parse::<u32>().expect("Level not a number"))
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn is_level_safe(a: u32, b: u32, should_increase: bool) -> bool {
    let diff = a.abs_diff(b);
    let diff_ok = diff >= 1 && diff <= 3;
    let increase_ok = (a < b) == should_increase;
    diff_ok && increase_ok
}

fn is_report_safe(report: &Vec<u32>) -> bool {
    // TODO: Support Problem Dampener

    if report.len() < 2 {
        return true;
    }

    let should_increase = report[0] < report[1];
    report
        .windows(2)
        .all(|window| is_level_safe(window[0], window[1], should_increase))
}

fn count_safe_reports(reports: &Vec<Vec<u32>>) -> usize {
    reports.iter().filter(|r| is_report_safe(r)).count()
}

fn main() {
    let reports = read_reports("input/day02.txt");
    let safe_count = count_safe_reports(&reports);
    println!("{}", safe_count);
}
