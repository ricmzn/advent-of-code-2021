use std::{error::Error, fs::read_to_string};

pub fn run() -> Result<(), Box<dyn Error>> {
    let data = read_to_string("day3.txt")?;
    println!(
        "day 3, part 1: power consumption: {}",
        get_power_consumption(&data)
    );
    println!(
        "day 3, part 2: life support rating: {}",
        get_life_support_rating(&data)
    );
    Ok(())
}

fn nth_bit(num: i32, bit: u32) -> bool {
    match num & (1 << bit) {
        0 => false,
        _ => true,
    }
}

fn set_nth_bit(num: &mut i32, bit: u32) {
    *num = *num | (1 << bit);
}

fn get_measurements(report: &str) -> Vec<i32> {
    report
        .trim()
        .lines()
        .map(|line| i32::from_str_radix(line.trim(), 2).unwrap())
        .collect()
}

fn get_measurement_size(report: &str) -> u32 {
    report
        .trim()
        .lines()
        .map(|line| line.trim().len() as u32)
        .nth(0)
        .unwrap()
}

fn most_common_bit(values: &[i32], bit: u32, default: bool) -> bool {
    let (ones, zeros) = values.iter().fold((0, 0), |(ones, zeros), value| {
        if nth_bit(*value, bit) == true {
            (ones + 1, zeros)
        } else {
            (ones, zeros + 1)
        }
    });
    if ones == zeros {
        default
    } else {
        ones > zeros
    }
}

fn get_power_consumption(report: &str) -> i32 {
    let measurements = get_measurements(report);
    let len = get_measurement_size(report);
    let mut gamma = 0;
    let mut epsilon = 0;
    for bit in 0..len {
        if most_common_bit(&measurements, bit, true) == true {
            set_nth_bit(&mut gamma, bit);
        } else {
            set_nth_bit(&mut epsilon, bit);
        }
    }
    gamma * epsilon
}

fn get_life_support_rating(report: &str) -> i32 {
    let measurements = get_measurements(report);
    let mut bit = get_measurement_size(report);
    let mut oxygen = measurements.clone();
    let mut co2 = measurements.clone();
    while oxygen.len() > 1 || co2.len() > 1 {
        bit = bit.checked_sub(1).expect("Calculation overflow");
        if oxygen.len() > 1 {
            let most_common = most_common_bit(&oxygen, bit, true);
            oxygen = oxygen
                .into_iter()
                .filter(|value| nth_bit(*value, bit) == most_common)
                .collect();
        }
        if co2.len() > 1 {
            let most_common = most_common_bit(&co2, bit, true);
            co2 = co2
                .into_iter()
                .filter(|value| nth_bit(*value, bit) != most_common)
                .collect();
        }
    }
    oxygen[0] * co2[0]
}

#[cfg(test)]
const INPUT: &str = r"
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010
";

#[test]
fn part1() {
    assert_eq!(get_power_consumption(INPUT), 198);
}

#[test]
fn part2() {
    assert_eq!(get_life_support_rating(INPUT), 230);
}
