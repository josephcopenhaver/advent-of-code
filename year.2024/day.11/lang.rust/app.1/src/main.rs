use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");
const NUM_ITERATIONS: usize = 25;

// note that this function assumes the input is never zero
fn len_in_base_10(x: usize) -> u32 {
    let mut count = 0;
    let mut x = x;
    while x != 0 {
        x /= 10;
        count += 1;
    }
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let size = INPUT.trim_end().chars().filter(|c| *c == ' ').count() + 1;

    // order does not matter, nor does sequence, just number of elements
    // so just need to track keys and counts

    let mut m = HashMap::<u64, u64>::with_capacity(size);
    let mut buf = HashMap::<u64, u64>::with_capacity(size);
    for v in INPUT.trim_end().split(" ") {
        let k = v.parse::<u64>()?;
        m.insert(k, m.get(&k).unwrap_or(0.borrow()) + 1);
    }

    for _ in 0..NUM_ITERATIONS {
        for (k, v) in m.iter() {
            let (k, v) = (*k, *v);

            let k = if k == 0 {
                1
            } else {
                let b10_size = len_in_base_10(k as usize);
                if b10_size % 2 == 1 {
                    k * 2024
                } else {
                    let mask = 10usize.pow(b10_size / 2);

                    let left_key = k / mask as u64;
                    buf.insert(left_key, *buf.get(&left_key).unwrap_or(0.borrow()) + v);

                    k % mask as u64
                }
            };
            buf.insert(k, *buf.get(&k).unwrap_or(0.borrow()) + v);
        }

        m.clear();
        (m, buf) = (buf, m);
    }

    let mut sum = 0;
    for v in m.values() {
        sum += *v;
    }

    println!("{}", sum);
    Ok(())
}
