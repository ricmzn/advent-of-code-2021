use std::{error::Error, fs::read_to_string};

pub fn run() -> Result<(), Box<dyn Error>> {
    let data = read_to_string("day6.txt")?;
    println!(
        "day 6, part 1: lanternfish count after 80 days: {}",
        count_lanternfish(&data, 80)
    );
    println!(
        "day 6, part 2: lanternfish count after 256 days: {}",
        count_lanternfish(&data, 256)
    );
    Ok(())
}

fn tick(ages: &mut [i64]) {
    let spawns = ages[0];
    for age in 0..8 {
        ages[age] = ages[age + 1];
    }
    ages[6] += spawns;
    ages[8] = spawns;
}

fn get_fish_ages(fish: &str) -> Vec<i64> {
    let mut ages = vec![0; 9];
    let fish_ages: Vec<usize> = fish
        .trim()
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    for age in fish_ages {
        ages[age] += 1;
    }
    ages
}

fn count_lanternfish(fish: &str, days: i32) -> i64 {
    let mut ages = get_fish_ages(fish);
    for _ in 0..days {
        tick(&mut ages);
    }
    ages.iter().sum()
}

#[cfg(test)]
const INPUT: &str = "3,4,3,1,2";

#[test]
fn part1() {
    assert_eq!(count_lanternfish(INPUT, 80), 5934);
}

#[test]
fn part2() {
    assert_eq!(count_lanternfish(INPUT, 256), 26984457539);
}
