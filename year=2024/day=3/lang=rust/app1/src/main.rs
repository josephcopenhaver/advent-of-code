use std::error::Error;

use regex::Regex;

const INPUT: &str = include_str!("../../../input.txt");

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((-?[1-9][0-9]*),(-?[1-9][0-9]*)\)").expect("lazy regex compile failed");
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for m in RE.captures_iter(INPUT) {
        sum += m[1].parse::<i32>()? * m[2].parse::<i32>()?;
    }

    println!("{}", sum);
    Ok(())
}
