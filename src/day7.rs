use std::{collections::HashMap, error::Error, fs::read_to_string};

pub fn run() -> Result<(), Box<dyn Error>> {
    let data = read_to_string("day7.txt")?;
    println!(
        "day 7, part 1: minimum fuel for alignment: {}",
        calculate_alignment_fuel(&data, false)
    );
    println!(
        "day 7, part 2: minimum fuel for alignment, with increasing cost: {}",
        calculate_alignment_fuel(&data, true)
    );
    Ok(())
}

fn get_crab_positions(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn average(list: &[i32]) -> i32 {
    f32::round(list.iter().sum::<i32>() as f32 / list.len() as f32) as i32
}

fn get_fuel_consumption(
    crabs: &[i32],
    center: i32,
    increasing_cost: bool,
    buffer: &mut HashMap<i32, i32>,
) -> i32 {
    if increasing_cost {
        crabs
            .iter()
            .map(|crab| {
                let dist = i32::abs(crab - center);
                buffer.get(&dist).cloned().unwrap_or_else(|| {
                    let mut sum = dist;
                    for i in 0..dist {
                        sum += i;
                    }
                    buffer.insert(dist, sum);
                    sum
                })
            })
            .sum()
    } else {
        crabs.iter().map(|crab| i32::abs(crab - center)).sum()
    }
}

fn find_best_center(crabs: &[i32], increasing_cost: bool) -> (i32, i32) {
    let mut buffer = HashMap::new();
    let mut pos = average(&crabs);
    let mut lowest = get_fuel_consumption(crabs, pos, increasing_cost, &mut buffer);

    let mut next = get_fuel_consumption(crabs, pos - 1, increasing_cost, &mut buffer);
    while next < lowest {
        lowest = next;
        pos -= 1;
        next = get_fuel_consumption(crabs, pos - 1, increasing_cost, &mut buffer);
    }

    let mut next = get_fuel_consumption(crabs, pos + 1, increasing_cost, &mut buffer);
    while next < lowest {
        lowest = next;
        pos += 1;
        next = get_fuel_consumption(crabs, pos + 1, increasing_cost, &mut buffer);
    }

    (pos, lowest)
}

fn calculate_alignment_fuel(input: &str, increasing_cost: bool) -> i32 {
    let crabs = get_crab_positions(input);
    let (_, fuel) = find_best_center(&crabs, increasing_cost);
    fuel
}

#[cfg(test)]
const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

#[test]
fn part1() {
    assert_eq!(calculate_alignment_fuel(INPUT, false), 37);
}

#[test]
fn part2() {
    assert_eq!(calculate_alignment_fuel(INPUT, true), 168);
}
