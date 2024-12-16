use std::{collections::HashMap, error::Error};

const INPUT: &str = include_str!("../../../input.txt");

const STEP_COUNT: i32 = 10000;
const W: i32 = 101;
const H: i32 = 103;

fn main() -> Result<(), Box<dyn Error>> {
    let size = INPUT
        .as_bytes()
        .iter()
        .cloned()
        .filter(|c| *c == b'\n')
        .count()
        + 1;
    let mut pos = vec![(-1, -1); size];
    let mut vel = vec![(0, 0); size];
    for (i, v) in INPUT.lines().enumerate() {
        let (p, v) = v.split_once(" ").expect("missing space delimiter");

        let p = &p[(p.find("=").expect("missing =") + 1)..]
            .split_once(",")
            .expect("missing ,");
        let v = &v[(v.find("=").expect("missing =") + 1)..]
            .split_once(",")
            .expect("missing ,");

        let p = (p.0.parse::<i32>()?, p.1.parse::<i32>()?);
        let v = (v.0.parse::<i32>()?, v.1.parse::<i32>()?);

        pos[i] = p;
        vel[i] = v;
    }

    let mut hm_vert = HashMap::<i32, usize>::new();
    let mut hm_horiz = HashMap::<i32, usize>::new();

    let mut max_alignment_step_id = -1;
    let mut max_alignment_score = 0;
    for step_id in 0..STEP_COUNT {
        hm_vert.clear();
        hm_horiz.clear();

        for i in 0..pos.len() {
            let p = &mut pos[i];

            let v = vel[i as usize];
            let mut x = (p.0 + v.0) % W;
            if x < 0 {
                x += W;
            }
            let mut y = (p.1 + v.1) % H;
            if y < 0 {
                y += H;
            }

            if let Some(v) = hm_vert.get(&x) {
                hm_vert.insert(x, *v + 1);
            } else {
                hm_vert.insert(x, 1);
            }

            if let Some(v) = hm_horiz.get(&y) {
                hm_horiz.insert(y, *v + 1);
            } else {
                hm_horiz.insert(y, 1);
            }

            *p = (x, y);
        }

        let alignment_score = hm_vert.values().max().expect("no vertical sets")
            + hm_horiz.values().max().expect("no vertical sets");
        if alignment_score > max_alignment_score {
            max_alignment_score = alignment_score;
            max_alignment_step_id = step_id + 1;
        }
    }

    println!("{}", max_alignment_step_id);
    Ok(())
}
