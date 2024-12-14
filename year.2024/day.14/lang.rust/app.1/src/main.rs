use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");

const STEP_COUNT: i32 = 100;
const W: i32 = 101;
const H: i32 = 103;

fn main() -> Result<(), Box<dyn Error>> {
    let mut sums = vec![vec![0; 2]; 2];
    for v in INPUT.lines() {
        let (p, v) = v.split_once(" ").expect("missing space delimiter");

        let p = &p[(p.find("=").expect("missing =") + 1)..]
            .split_once(",")
            .expect("missing ,");
        let v = &v[(v.find("=").expect("missing =") + 1)..]
            .split_once(",")
            .expect("missing ,");

        let p = (p.0.parse::<i32>()?, p.1.parse::<i32>()?);
        let v = (v.0.parse::<i32>()?, v.1.parse::<i32>()?);

        let mut x = (p.0 + STEP_COUNT * v.0) % W;
        if x < 0 {
            x += W;
        }
        x = if x == W / 2 {
            continue;
        } else if x < W / 2 {
            0
        } else {
            1
        };

        let mut y = (p.1 + STEP_COUNT * v.1) % H;
        if y < 0 {
            y += H;
        }
        y = if y == H / 2 {
            continue;
        } else if y < H / 2 {
            0
        } else {
            1
        };

        sums[y as usize][x as usize] += 1;
    }

    let mut safety_factor = 1;
    for v in sums {
        for v in v {
            safety_factor *= v
        }
    }

    println!("{}", safety_factor);
    Ok(())
}
