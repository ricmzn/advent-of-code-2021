use std::{error::Error, fs::read_to_string};

pub fn run() -> Result<(), Box<dyn Error>> {
    let data = read_to_string("day2.txt")?;
    println!(
        "day 2, part 1: position index: {}",
        get_position_index(&data)
    );
    println!(
        "day 2, part 2: position index + aim: {}",
        get_position_index_aim(&data)
    );
    Ok(())
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn get_commands(plan: &str) -> Vec<Command> {
    plan.trim()
        .lines()
        .map(|line| {
            let (cmd, arg) = line.trim().split_once(" ").unwrap();
            let arg: i32 = arg.parse().unwrap();
            match cmd {
                "forward" => Command::Forward(arg),
                "down" => Command::Down(arg),
                "up" => Command::Up(arg),
                cmd => panic!("Unknown argument: {}", cmd),
            }
        })
        .collect()
}

fn get_position_index(plan: &str) -> i32 {
    let mut forward = 0;
    let mut depth = 0;
    let commands = get_commands(plan);
    for cmd in commands {
        match cmd {
            Command::Forward(arg) => forward += arg,
            Command::Down(arg) => depth += arg,
            Command::Up(arg) => depth -= arg,
        }
    }
    forward * depth
}

fn get_position_index_aim(plan: &str) -> i32 {
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;
    let commands = get_commands(plan);
    for cmd in commands {
        match cmd {
            Command::Forward(arg) => {
                forward += arg;
                depth += arg * aim;
            },
            Command::Down(arg) => aim += arg,
            Command::Up(arg) => aim -= arg,
        }
    }
    forward * depth
}

#[cfg(test)]
const INPUT: &str = r"
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
";

#[test]
fn part1() {
    assert_eq!(get_position_index(INPUT), 150);
}

#[test]
fn part2() {
    assert_eq!(get_position_index_aim(INPUT), 900);
}

