use std::{error::Error, fs::read_to_string};

pub fn run() -> Result<(), Box<dyn Error>> {
    let data = read_to_string("day1.txt")?;
    println!(
        "day 1, part 1: depth increases: {}",
        count_depth_increases(&data)
    );
    println!(
        "day 1, part 2: using windows: {}",
        count_depth_increase_windows(&data)
    );
    Ok(())
}

fn get_measurements(report: &str) -> Vec<i32> {
    report
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn count_depth_increases(report: &str) -> i32 {
    let mut increases = 0;
    let measurements = get_measurements(report);
    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            increases += 1;
        }
    }
    increases
}

fn count_depth_increase_windows(report: &str) -> i32 {
    let mut increases = 0;
    let measurements = get_measurements(report);
    for i in 3..measurements.len() {
        let previous_window = measurements[i - 3] + measurements[i - 2] + measurements[i - 1];
        let current_window = measurements[i - 2] + measurements[i - 1] + measurements[i - 0];
        if current_window > previous_window {
            increases += 1;
        }
    }
    increases
}

#[cfg(test)]
const INPUT: &str = r"
    199
    200
    208
    210
    200
    207
    240
    269
    260
    263
";

#[test]
fn part1() {
    assert_eq!(count_depth_increases(INPUT), 7);
}

#[test]
fn part2() {
    assert_eq!(count_depth_increase_windows(INPUT), 5);
}
