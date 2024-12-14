use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    error::Error,
    rc::Rc,
};

struct Bitmap(Vec<u8>);

// Bitmap is quick and dirty
// it has zero bounds checking
// and can end in up to 7 unused bits
//
// Ideally a full implementation would have
// these aspects covered.
impl Bitmap {
    fn new(size: impl Into<usize>) -> Bitmap {
        let size = size.into();

        Bitmap(vec![0; (size + 7) / 8])
    }

    fn set<T: Into<usize>>(&mut self, idx: T) -> bool {
        let idx = idx.into();

        let ptr = &mut self.0[idx / 8];
        let changed = ((*ptr) & (1 << (idx % 8))) == 0;
        *ptr |= 1 << (idx % 8);
        changed
    }
}

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
                        // short circuit if the candidate node is already in the node above's set
                        let rc_set_a = equiv_sets
                            .get(&np)
                            .expect("must exist: above entry")
                            .clone();
                        let set_a = rc_set_a.borrow();
                        if set_a.contains(&p) {
                            continue;
                        }

                        // merge the sets into the larger one or left
                        let rc_set_b = equiv_sets
                            .get(&(p.0 - 1, p.1))
                            .expect("must exist: left entry")
                            .clone();
                        let set_b = rc_set_b.borrow();

                        // minimize movement operations by finding the minimum size set to act as src

                        let merge_b_to_a = set_b.len() < set_a.len();
                        drop(set_b);
                        drop(set_a);

                        let (src, mut dst, rc_dst) = if merge_b_to_a {
                            (rc_set_b.borrow(), rc_set_a.borrow_mut(), &rc_set_a)
                        } else {
                            (rc_set_a.borrow(), rc_set_b.borrow_mut(), &rc_set_b)
                        };

                        // perform the union and moves the equivalence class requires
                        for v in src.iter().cloned() {
                            dst.insert(v);
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

    let mut visited = Bitmap::new(w * h * 4);
    macro_rules! mark_visited {
        ( $p:expr, $side_id:expr ) => {
            (|p: (i32, i32), d: usize| {
                visited.set((p.1 as usize * w + p.0 as usize) * 4 + d as usize)
            })($p, $side_id)
        };
    }

    let dir_trans = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let orth_dirs = [
        [(0, -1), (0, 1)],
        [(0, -1), (0, 1)],
        [(-1, 0), (1, 0)],
        [(-1, 0), (1, 0)],
    ];

    let mut sum = 0;
    while let Some(rc_hs) = equiv_sets.values().nth(0) {
        let rc_hs = rc_hs.clone();
        let hs = rc_hs.borrow();
        let area = hs.len();

        // identify perimeter contributing nodes
        // for each perimeter node side exposed
        // raycast if the next node along the way has the same border configuration until end
        // mark each border as handled when the raycast overlaps
        // count each raycast regardless of length

        let mut perimeter = 0;
        for v in hs.iter().cloned() {
            equiv_sets.remove(&v);

            for (i, d) in dir_trans.iter().cloned().enumerate() {
                let p = (v.0 + d.0, v.1 + d.1);

                if hs.contains(&p) {
                    // the plot continues in this direction
                    //
                    // short circuit because there is no edge
                    continue;
                }

                // there is no continuation in this direction
                // and p could be off the grid

                // mark the edge as recorded
                // for the orthogonal directions, mark those edges as recorded as well while they are in the set and grid
                // and have the same edge characteristic
                //
                // if the edge is already marked then short circuit
                if !mark_visited!(v, i) {
                    continue;
                }

                perimeter += 1;

                for od in orth_dirs[i] {
                    let mut p = v;
                    loop {
                        p.0 += od.0;
                        p.1 += od.1;

                        // note that the check `!(p.0 >= 0 && p.1 >= 0 && (p.0 as usize) < w && (p.1 as usize) < h)`
                        // is not required here because a point off the grid is not going to be within the equivalence
                        // set - but it would be a faster operation if either w or y are 1 which should be super rare
                        // as it makes the problem trivial to solve

                        if !hs.contains(&p) {
                            break;
                        }

                        if hs.contains(&(p.0 + d.0, p.1 + d.1)) {
                            break;
                        }

                        // the point checked above is either off the grid
                        // or belongs to a different set
                        //
                        // mark edge as visited as part of the ray

                        mark_visited!(p, i);
                    }
                }
            }
        }

        sum += area * perimeter;
    }

    println!("{}", sum);
    Ok(())
}
