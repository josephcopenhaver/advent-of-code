const INPUT: &str = include_str!("../../../input.txt");

fn main() {
    let size = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;

    println!("{}", INPUT.lines().map(|v| 0).sum::<i32>());
}
