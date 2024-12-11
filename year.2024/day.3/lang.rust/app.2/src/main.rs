use std::error::Error;

use regex::Regex;

const INPUT: &str = include_str!("../../../input.txt");

const ON: &str = "do()";
const OFF: &str = "don't()";

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((-?[1-9][0-9]*),(-?[1-9][0-9]*)\)").expect("lazy regex compile failed");
}

fn main() -> Result<(), Box<dyn Error>> {
    let buf = match INPUT.split_once(OFF) {
        Some((valid, other)) => {
            let buf = String::with_capacity(INPUT.len());
            buf.push_str(valid);
            for v in other.split(ON).skip(1) {
                match v.split_once(OFF) {
                    Some((v, _)) => {
                        buf.push_str(" ");
                        buf.push_str(v);
                    }
                    None => {
                        buf.push_str(" ");
                        buf.push_str(v);
                    }
                }
            }

            buf
        }
        None => {
            INPUT.to_string();
        }
    };

    let mut sum = 0;
    for m in RE.captures_iter(&buf) {
        sum += m[1].parse::<i32>()? * m[2].parse::<i32>()?;
    }

    println!("{}", sum);
    Ok(())
}
