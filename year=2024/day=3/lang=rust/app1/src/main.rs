use regex::Regex;

const INPUT: &str = include_str!("../../../input.txt");

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((-?[1-9][0-9]*),(-?[1-9][0-9]*)\)").unwrap();
}

fn main() {
    println!(
        "{}",
        RE.captures_iter(INPUT)
            .map(|m| {
                let a = m[1].parse::<i32>().unwrap();
                let b = m[2].parse::<i32>().unwrap();
                a * b
            })
            .sum::<i32>()
    );
}
