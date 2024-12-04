const INPUT: &str = include_str!("../../../input.txt");

const NEEDLE: &str = "MAS";
const NEEDLE_REV: &str = "SAM";

fn main() {
    // note: only works when grid w and h are at least NEEDLE length

    let h = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;
    let w = INPUT.find("\n").unwrap();

    let mut grid = Vec::<&str>::with_capacity(h);
    INPUT.lines().for_each(|l| grid.push(l));

    let mut buf = String::with_capacity(NEEDLE.len());
    let mut sum = 0;

    // diagonals
    for y in 0..(h - NEEDLE.len() + 1) {
        for x in 0..(w - NEEDLE.len() + 1) {
            // positive y trend
            buf.clear();
            for d in 0..NEEDLE.len() {
                buf.push(grid[y + d].as_bytes()[x + d] as char);
            }
            if !(buf.eq(NEEDLE) || buf.eq(NEEDLE_REV)) {
                continue;
            }

            // negative y trend
            buf.clear();
            for d in 0..NEEDLE.len() {
                buf.push(grid[y + NEEDLE.len() - 1 - d].as_bytes()[x + d] as char);
            }
            if buf.eq(NEEDLE) || buf.eq(NEEDLE_REV) {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}
