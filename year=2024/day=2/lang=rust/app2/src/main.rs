const INPUT: &str = include_str!("../../../input.txt");

fn increasing(x: &Vec<i32>) -> bool {
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

fn decreasing(x: &Vec<i32>) -> bool {
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

fn main() {
    println!(
        "{}",
        INPUT
            .lines()
            .map(|v| {
                let x: Vec<i32> = v
                    .split_whitespace()
                    .map(|f| f.parse::<i32>().unwrap())
                    .collect();
                if increasing(&x) || decreasing(&x) {
                    return 1;
                }
                for i in 0..x.len() {
                    let mut x = x.clone();
                    x.remove(i);
                    if increasing(&x) || decreasing(&x) {
                        return 1;
                    }
                }
                return 0;
            })
            .sum::<i32>()
    );
}
