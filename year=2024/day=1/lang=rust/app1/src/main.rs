use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let size = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;

    let mut left = Vec::<i32>::with_capacity(size);
    let mut right = Vec::<i32>::with_capacity(size);
    for v in INPUT.lines() {
        let (l, r) = v
            .split_once("   ")
            .expect("record not separated by 3 spaces");
        left.push(l.parse::<i32>()?);
        right.push(r.parse::<i32>()?);
    }
    left.sort_unstable();
    right.sort_unstable();

    let mut sum = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        sum += (l - r).abs();
    }

    println!("{}", sum);
    Ok(())
}
