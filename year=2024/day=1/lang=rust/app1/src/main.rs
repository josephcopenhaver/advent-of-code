const INPUT: &str = include_str!("../../../input.txt");

fn main() {
    let size = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;

    let mut left = Vec::<i32>::with_capacity(size);
    let mut right = Vec::<i32>::with_capacity(size);
    INPUT.lines().for_each(|v| {
        let mut it = v.split_whitespace();
        left.push(it.next().unwrap().parse::<i32>().unwrap());
        right.push(it.next().unwrap().parse::<i32>().unwrap());
    });
    left.sort_unstable();
    right.sort_unstable();

    println!(
        "{}",
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum::<i32>()
    );
}
