use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");

fn mono_inc(x: &[i32]) -> bool {
    let mut prev = x[0];
    for &n in x.iter().skip(1) {
        if n <= prev {
            return false;
        }
        if n - prev > 3 {
            return false;
        }
        prev = n;
    }
    true
}

fn mono_dec(x: &[i32]) -> bool {
    let mut prev = x[0];
    for &n in x.iter().skip(1) {
        if n >= prev {
            return false;
        }
        if prev - n > 3 {
            return false;
        }
        prev = n;
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let size = INPUT
        .lines()
        .map(|v| v.chars().filter(|&v| v == ' ').count())
        .max()
        .expect("no input")
        + 1;

    let mut buf: Vec<i32> = Vec::<i32>::with_capacity(size);

    let mut sum = 0;
    for v in INPUT.lines() {
        buf.clear();
        for v in v.split(" ") {
            buf.push(v.parse::<i32>()?);
        }

        if mono_inc(&buf) || mono_dec(&buf) {
            sum += 1;
            continue;
        }
    }

    println!("{}", sum);
    Ok(())
}
