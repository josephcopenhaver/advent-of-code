const INPUT: &str = include_str!("../../../input.txt");

const NEEDLE: &str = "MAS";

fn main() {
    let needle_rev = &NEEDLE.chars().rev().collect::<String>();
    let needle_idx_shift = NEEDLE.len() - 1;
    // note: only works when grid w and h are at least NEEDLE length

    let h = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;
    let w = INPUT.find("\n").unwrap();

    let mut grid = Vec::<&str>::with_capacity(h);
    INPUT.lines().for_each(|l| grid.push(l));

    let mut buf = String::with_capacity(NEEDLE.len());
    let mut sum = 0;

    // diagonals
    for y in 0..(h - needle_idx_shift) {
        let negative_y_trend = y + needle_idx_shift;
        for x in 0..(w - needle_idx_shift) {
            // positive y trend
            buf.clear();
            for d in 0..NEEDLE.len() {
                buf.push(grid[y + d].as_bytes()[x + d] as char);
            }
            if !(buf.eq(NEEDLE) || buf.eq(needle_rev)) {
                continue;
            }

            // negative y trend
            buf.clear();
            for d in 0..NEEDLE.len() {
                buf.push(grid[negative_y_trend - d].as_bytes()[x + d] as char);
            }
            if buf.eq(NEEDLE) || buf.eq(needle_rev) {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}
