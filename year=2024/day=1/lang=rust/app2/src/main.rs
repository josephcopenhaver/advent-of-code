use std::{borrow::Borrow, collections::HashMap};

const INPUT: &str = include_str!("../../../input.txt");

fn main() {
    let size = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;

    let mut left = Vec::<i32>::with_capacity(size + 1);
    left.push(0);
    let mut map: HashMap<i32, i32> = HashMap::new();
    INPUT.lines().for_each(|v| {
        let mut it = v.split_whitespace();
        left.push(it.next().unwrap().parse::<i32>().unwrap());
        let k = it.next().unwrap().parse::<i32>().unwrap();
        if let Some(v) = map.get(&k) {
            map.insert(k, v + k);
            return;
        }
        map.insert(k, k);
    });

    println!(
        "{}",
        left.into_iter()
            .reduce(|sum, k| sum + map.get(&k).unwrap_or(0.borrow()))
            .unwrap()
    );
}
