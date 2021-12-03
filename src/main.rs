use std::error::Error;

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), Box<dyn Error>> {
    let day = 3;
    match day {
        1 => day1::run()?,
        2 => day2::run()?,
        3 => day3::run()?,
        _ => Err(format!("Invalid day: {}", day))?,
    };
    Ok(())
}
