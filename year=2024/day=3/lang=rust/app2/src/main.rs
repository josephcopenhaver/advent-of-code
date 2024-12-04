use regex::Regex;

const INPUT: &str = include_str!("../../../input.txt");

const ON: &str = "do()";
const OFF: &str = "don't()";

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((-?[1-9][0-9]*),(-?[1-9][0-9]*)\)").unwrap();
}

fn main() {
    let mut buf = String::with_capacity(INPUT.len());
    match INPUT.split_once(OFF) {
        Some((valid, other)) => {
            buf.push_str(valid);
            other
                .split(ON)
                .skip(1)
                .for_each(|v| match v.split_once(OFF) {
                    Some((v, _)) => {
                        buf.push_str(" ");
                        buf.push_str(v);
                    }
                    None => {
                        buf.push_str(" ");
                        buf.push_str(v);
                    }
                });
        }
        None => {
            buf = INPUT.to_string();
        }
    }

    println!(
        "{}",
        RE.captures_iter(&buf)
            .map(|m| {
                let a = m[1].parse::<i32>().unwrap();
                let b = m[2].parse::<i32>().unwrap();
                a * b
            })
            .sum::<i32>()
    );
}
