use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let size = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;

    let mut sum = 0;
    for v in INPUT.lines() {
        sum += v.parse::<i32>()?;
    }

    println!("{}", sum);
    Ok(())
}
