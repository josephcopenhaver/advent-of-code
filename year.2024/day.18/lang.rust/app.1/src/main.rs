use std::{collections::HashMap, error::Error};

const INPUT: &str = include_str!("../../../input.txt");

// TODO: implement solution using A*

struct Bitmap {
    m: Vec<u8>,
}

// Bitmap is quick and dirty
// it has zero bounds checking
// and can end in up to 7 unused bits
//
// Ideally a full implementation would have
// these aspects covered.
impl Bitmap {
    fn new(size: impl Into<usize>) -> Bitmap {
        let size = size.into();

        Bitmap {
            // size: size,
            m: vec![0; (size + 7) / 8],
        }
    }

    fn set<T: Into<usize>>(&mut self, idx: T) -> bool {
        let idx = idx.into();

        let ptr = &mut self.m[idx / 8];
        let changed = ((*ptr) & (1 << (idx % 8))) == 0;
        *ptr |= 1 << (idx % 8);
        changed
    }

    fn is_set<T: Into<usize>>(&self, idx: T) -> bool {
        let idx = idx.into();

        ((self.m[idx / 8]) & (1 << (idx % 8))) != 0
    }
}

const MAX_CORRUPTION: usize = 1024;
const GOAL_X: i8 = 70;
const GOAL_Y: i8 = 70;

fn main() -> Result<(), Box<dyn Error>> {
    let (w, h) = (GOAL_X + 1, GOAL_Y + 1);
    let point_to_idx = |(x, y): (i8, i8)| (y as usize) * (w as usize) + (x as usize);
    let idx_to_point = |v: usize| ((v % (w as usize)) as i8, (v / (w as usize)) as i8);

    let mut visited = Bitmap::new(w as usize * h as usize);

    // read up to first kb into map
    for (i, v) in INPUT.trim_end().split("\n").enumerate() {
        if i == MAX_CORRUPTION {
            break;
        }
        let (x, y) = v.split_once(",").unwrap();
        let (x, y) = (x.parse::<i8>().unwrap(), y.parse::<i8>().unwrap());
        visited.set(point_to_idx((x, y)));
    }

    let mut origins = HashMap::<usize, usize>::new();
    origins.insert(0, 0);
    visited.set(0 as usize);

    let mut exhausted_origins = Vec::<usize>::new();

    let goal_idx = point_to_idx((GOAL_X, GOAL_Y));
    while !origins.is_empty() && !visited.is_set(goal_idx) {
        let mut next: Option<(usize, usize)> = None;

        // TODO: can optimize storing origins in distance order
        // and only traverse those at the minimum distance
        for (&idx, &dist) in origins.iter() {
            let ndist = dist + 1;
            if let Some((_, v)) = next.clone() {
                if ndist >= v {
                    continue;
                }
            }

            let p = idx_to_point(idx);
            let mut exhausted = true;
            for v in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let np = (p.0 + v.0, p.1 + v.1);
                if np.0 < 0 || np.1 < 0 || np.0 >= w || np.1 >= h {
                    continue;
                }

                let nidx = point_to_idx(np);
                if visited.is_set(nidx) {
                    continue;
                }

                exhausted = false;

                if let Some(v) = &mut next {
                    if v.1 > ndist {
                        v.0 = nidx;
                        v.1 = ndist;
                    }
                    break;
                }

                next = Some((nidx, ndist));
                break;
            }

            if exhausted {
                exhausted_origins.push(idx);
            }
        }

        for v in exhausted_origins.iter().cloned() {
            origins.remove(&v);
        }
        exhausted_origins.clear();

        let (idx, dist) = if let Some(v) = next {
            v
        } else {
            break;
        };

        origins.insert(idx, dist);
        visited.set(idx);
    }

    println!("{}", origins.get(&goal_idx).expect("no solution found"));
    Ok(())
}
