// core concepts:
// - https://en.wikipedia.org/wiki/System_of_linear_equations

use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");

// formulas:
//
// cost = 3a + b
//
// system of linear equations:
//
// 1. 94*a + 22*b = 8400
// 2. 34*a + 67*b = 5400
//
// 1.2 a = (8400 - 22*b)/94
// 1.3 34*(8400 - 22*b) + 67*94*b = 5400*94
// 1.4 8400*34 - 22*34*b + 67*94*b = 5400*94
// 1.5 67*94*b - 22*34*b = 5400*94 - 8400*34
// 1.5 b = (5400*94 - 8400*34)/(67*94 - 22*34)
// 2.2 a = (5400 - 67*b)/34

fn parse_line(s: &str, sep: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let (mut x, mut y) = s.split_once(", ").expect("missing ', ' delimiter");
    x = &x[(x.find(sep).expect("missing first separator") + 1)..];
    y = &y[(y.find(sep).expect("missing second separator") + 1)..];

    Ok((x.parse::<i32>()?, y.parse::<i32>()?))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for v in INPUT.split("\n\n") {
        let (a, b) = v.split_once("\n").expect("missing first line");
        let (b, p) = b.split_once("\n").expect("missing second line");
        let a = parse_line(a, "+")?;
        let b = parse_line(b, "+")?;
        let p = parse_line(p, "=")?;

        let second_b_term = b.1;

        // solve b in the system of linear equations
        let mut numerator = p.1 * a.0 - p.0 * a.1;
        let mut denom = second_b_term * a.0 - b.0 * a.1;
        if numerator % denom != 0 {
            continue;
        }

        let b = numerator / denom;

        // solve a in the system of linear equations
        numerator = p.1 - b * second_b_term;
        denom = a.1;
        if numerator % denom != 0 {
            continue;
        }

        let a = numerator / denom;

        sum += a * 3 + b;
    }

    println!("{}", sum);
    Ok(())
}
