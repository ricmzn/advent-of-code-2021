use std::error::Error;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 6;
    match day {
        1 => day1::run()?,
        2 => day2::run()?,
        3 => day3::run()?,
        4 => day4::run()?,
        5 => day5::run()?,
        6 => day6::run()?,
        _ => Err(format!("Invalid day: {}", day))?,
    };
    Ok(())
}
