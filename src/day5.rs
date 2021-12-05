use std::{error::Error, fs::read_to_string};

pub fn run() -> Result<(), Box<dyn Error>> {
    let data = read_to_string("day5.txt")?;
    println!(
        "day 5, part 1: dangerous points (minus diagonals): {}",
        get_dangerous_points(&data, false)
    );
    println!(
        "day 5, part 2: dangerous points (with diagonals): {}",
        get_dangerous_points(&data, true)
    );
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_str(point: &str) -> Self {
        let (x, y) = point.trim().split_once(",").unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }

    fn from_line(line: &str) -> (Self, Self) {
        let (from, to) = line.trim().split_once("->").unwrap();
        (Self::from_str(from), Self::from_str(to))
    }
}

#[derive(Debug)]
struct VentMap {
    width: i32,
    density: Vec<i32>,
}

impl VentMap {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            density: vec![0; usize::try_from(width * height).unwrap()],
        }
    }

    fn add_line(&mut self, from: Point, to: Point) {
        let xdir = if to.x > from.x { 1 } else { -1 };
        let ydir = if to.y > from.y { 1 } else { -1 };
        let mut x = from.x;
        let mut y = from.y;
        loop {
            let idx = x * self.width + y;
            self.density[idx as usize] += 1;
            if x == to.x && y == to.y {
                break;
            } else {
                if x != to.x {
                    x += xdir;
                }
                if y != to.y {
                    y += ydir;
                }
            }
        }
    }

    fn from_data(data: &str, diagonals: bool) -> Self {
        let lines: Vec<_> = data
            .trim()
            .lines()
            .map(|line| Point::from_line(line))
            .collect();

        let xmax = lines
            .iter()
            .flat_map(|(from, to)| [from.x, to.x])
            .max()
            .unwrap();

        let ymax = lines
            .iter()
            .flat_map(|(from, to)| [from.y, to.y])
            .max()
            .unwrap();

        let mut map = Self::new(xmax + 1, ymax + 1);
        for (from, to) in lines {
            if diagonals || from.x == to.x || from.y == to.y {
                map.add_line(from, to);
            }
        }

        map
    }
}

fn get_dangerous_points(data: &str, diagonals: bool) -> i32 {
    VentMap::from_data(data, diagonals)
        .density
        .into_iter()
        .filter(|density| *density >= 2)
        .count() as i32
}

#[cfg(test)]
const INPUT: &str = r"
    0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
";

#[test]
fn part1() {
    assert_eq!(get_dangerous_points(INPUT, false), 5);
}

#[test]
fn part2() {
    assert_eq!(get_dangerous_points(INPUT, true), 12);
}
