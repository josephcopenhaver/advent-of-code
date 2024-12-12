use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    error::Error,
    rc::Rc,
};

const INPUT: &str = include_str!("../../../input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let w = INPUT.find("\n").expect("missing newlines");
    let h = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;
    let mut grid = vec![vec![0; w]; h];
    for (y, v) in INPUT.lines().enumerate() {
        for (x, v) in v.as_bytes().iter().cloned().enumerate() {
            grid[y][x] = v;
        }
    }

    let mut equiv_sets = HashMap::<(i32, i32), Rc<RefCell<HashSet<(i32, i32)>>>>::new();
    for y in 0..h {
        for x in 0..w {
            let p = (x as i32, y as i32);
            let mut merged_at_least_once = false;
            for d in [(-1, 0), (0, -1)] {
                let np = (p.0 + d.0, p.1 + d.1);
                if np.0 < 0 || np.1 < 0 {
                    continue;
                }
                if grid[np.1 as usize][np.0 as usize] == grid[y][x] {
                    if merged_at_least_once {
                        // short circuit if the candidate node is already in the src node we're going to merge into dst
                        let rc_src = equiv_sets
                            .get(&np)
                            .expect("must exist: above entry")
                            .clone();
                        let src = rc_src.borrow();
                        if src.contains(&p) {
                            continue;
                        }

                        // merge the sets left and above into left
                        let rc_dst = equiv_sets
                            .get(&(p.0 - 1, p.1))
                            .expect("must exist: left entry")
                            .clone();
                        let mut dst = rc_dst.borrow_mut();
                        for v in src.iter().cloned() {
                            dst.insert(v);
                        }
                        for v in src.iter().cloned() {
                            equiv_sets.insert(v, rc_dst.clone());
                        }
                        continue;
                    }
                    merged_at_least_once = true;
                    let rc_hs = equiv_sets
                        .get(&np)
                        .expect("no previous equiv set decision")
                        .clone();
                    let mut hs = rc_hs.borrow_mut();
                    hs.insert(p);
                    equiv_sets.insert(p, rc_hs.clone());
                }
            }
            if merged_at_least_once {
                continue;
            }

            let mut hs = HashSet::<(i32, i32)>::with_capacity(1);
            hs.insert(p);
            equiv_sets.insert(p, Rc::new(RefCell::new(hs)));
        }
    }

    let mut sum = 0;
    while let Some(rc_hs) = equiv_sets.values().nth(0) {
        let rc_hs = rc_hs.clone();
        let hs = rc_hs.borrow();
        let area = hs.len();

        for v in hs.iter().cloned() {
            equiv_sets.remove(&v);
        }

        let mut perimiter = 0;
        for v in hs.iter().cloned() {
            for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let p = (v.0 + d.0, v.1 + d.1);
                if p.0 < 0 || p.1 < 0 || p.0 >= w as i32 || p.1 >= h as i32 {
                    perimiter += 1;
                    continue;
                }
                if !hs.contains(&p) {
                    perimiter += 1;
                }
            }
        }

        sum += area * perimiter;
    }

    println!("{}", sum);
    Ok(())
}
