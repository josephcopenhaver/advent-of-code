use std::{borrow::Borrow, collections::HashMap, error::Error};

const INPUT: &str = include_str!("../../../input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let size = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;

    let mut left = Vec::<i32>::with_capacity(size);
    let mut map: HashMap<i32, i32> = HashMap::new();
    for v in INPUT.lines() {
        let (v, k) = v
            .split_once("   ")
            .expect("record not separated by 3 spaces");

        left.push(v.parse::<i32>()?);
        let k = k.parse::<i32>()?;

        if let Some(v) = map.get(&k) {
            map.insert(k, v + k);
            continue;
        }

        map.insert(k, k);
    }

    let mut sum = 0;
    for k in left {
        sum += map.get(&k).unwrap_or(0.borrow());
    }

    println!("{}", sum);
    Ok(())
}
