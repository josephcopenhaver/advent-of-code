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

    fn clear(&mut self) {
        for i in 0..self.m.len() {
            self.m[i] = 0;
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

    let mut origins = HashMap::<usize, usize>::new();
    let mut exhausted_origins = Vec::<usize>::new();

    let mut input_nums =
        Vec::<i16>::with_capacity(INPUT.trim_end().chars().filter(|c| *c == ',').count());
    for v in INPUT.trim_end().split("\n") {
        let (x, y) = v.split_once(",").unwrap();
        let (x, y) = (x.parse::<i8>().unwrap(), y.parse::<i8>().unwrap());
        input_nums.push(point_to_idx((x, y)) as i16);
    }

    // TODO: could binary search rather than linearly scan
    // finishes in 10 seconds anyways so not gonna bother
    //
    // note that the goal of this is to identify at what position in the
    // sequence is there a partition between points reachable from 0,0 and 70,70
    //
    // this is not a path problem, it is an equivalence class problem - one that
    // can be trivially simplified by instead inverting the design and finding
    // the point in which a graph with all walls in place eventually has one
    // equivalence class of connectedness that includes 0,0 and 70,70 as walls
    // are being removed in the reverse order they were added.
    //
    // this is solvable in O(n) time with a two linear scans of the points
    //
    // just not going to write it as my time is valuable

    let mut max_corruption = MAX_CORRUPTION;
    let mut last_iter = false;
    let mut answer = None;
    while !last_iter {
        max_corruption += 1;
        visited.clear();

        last_iter = true;
        for (i, idx) in input_nums.iter().cloned().enumerate() {
            if i == max_corruption {
                last_iter = false;
                break;
            }
            visited.set(idx as usize);
        }

        origins.clear();
        origins.insert(0, 0);
        visited.set(0 as usize);

        exhausted_origins.clear();

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

        if origins.get(&goal_idx).is_none() {
            answer = Some(input_nums[max_corruption - 1]);
            break;
        }
    }

    let answer = idx_to_point(if let Some(v) = answer {
        v as usize
    } else {
        panic!("no solution found");
    });

    println!("{},{}", answer.0, answer.1);

    Ok(())
}
