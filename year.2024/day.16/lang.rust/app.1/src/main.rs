use std::{
    collections::{HashMap, HashSet},
    error::Error,
    i32, usize,
};

const INPUT: &str = include_str!("../../../input.txt");

// shortest path = use Dijkstra’s algorithm or A*

// TODO: implement solution using A*

struct Bitmap {
    size: usize,
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
            size: size,
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

    // fn unset<T: Into<usize>>(&mut self, idx: T) -> bool {
    //     let idx = idx.into();
    //     let ptr = &mut self.m[idx / 8];
    //     let changed = ((*ptr) & (1 << (idx % 8))) != 0;
    //     *ptr &= !(1 << (idx % 8));
    //     changed
    // }

    fn is_set<T: Into<usize>>(&self, idx: T) -> bool {
        let idx = idx.into();

        ((self.m[idx / 8]) & (1 << (idx % 8))) != 0
    }

    // fn clone(&self) -> Bitmap {
    //     let mut dst = Bitmap::new(self.size);
    //     for (i, v) in self.m.iter().cloned().enumerate() {
    //         dst.m[i] = v;
    //     }
    //     dst
    // }

    // fn clone_into(&self, dst: &mut Bitmap) -> bool {
    //     if dst.size != self.size {
    //         return false;
    //     }
    //     for (i, v) in self.m.iter().cloned().enumerate() {
    //         dst.m[i] = v;
    //     }
    //     true
    // }

    fn count_ones(&self) -> usize {
        self.m
            .iter()
            .cloned()
            .map(|x| x.count_ones() as usize)
            .sum()
    }

    // fn count_zeros(&self) -> usize {
    //     self.size - self.count_ones()
    // }

    fn iter_set<'a>(&'a self) -> BitmapIterSet<'a> {
        BitmapIterSet { bm: &self, next: 0 }
    }

    // fn iter_unset<'a>(&'a self) -> BitmapIterUnset<'a> {
    //     BitmapIterUnset { bm: &self, next: 0 }
    // }
}

struct BitmapIterSet<'a> {
    bm: &'a Bitmap,
    next: usize,
}

impl<'a> Iterator for BitmapIterSet<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut idx = self.next / 8;
        loop {
            if self.next >= self.bm.size {
                return None;
            }

            let v = self.bm.m[idx];
            if v == 0 {
                idx += 1;
                let old = self.next;
                self.next = idx << 3;
                if self.next < old {
                    self.next = usize::MAX;
                    return None;
                }

                continue;
            }

            let mut mask = 1 << (self.next % 8);
            if (v & !(mask - 1)) == 0 {
                idx += 1;
                let old = self.next;
                self.next = idx << 3;
                if self.next < old {
                    self.next = usize::MAX;
                    return None;
                }

                continue;
            }

            while (v & mask) == 0 {
                mask <<= 1;
                self.next += 1;
                if self.next >= self.bm.size {
                    return None;
                }
            }

            break;
        }

        let resp = self.next;
        self.next += 1;

        Some(resp)
    }
}

// struct BitmapIterUnset<'a> {
//     bm: &'a Bitmap,
//     next: usize,
// }

// impl<'a> Iterator for BitmapIterUnset<'a> {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         let mut idx = self.next / 8;
//         loop {
//             if self.next >= self.bm.size {
//                 return None;
//             }

//             let v = self.bm.m[idx];
//             if v == 0xFF {
//                 idx += 1;
//                 let old = self.next;
//                 self.next = idx << 3;
//                 if self.next < old {
//                     self.next = usize::MAX;
//                     return None;
//                 }

//                 continue;
//             }

//             let mut mask = 1 << (self.next % 8);
//             if (v & !(mask - 1)) == !(mask - 1) {
//                 idx += 1;
//                 let old = self.next;
//                 self.next = idx << 3;
//                 if self.next < old {
//                     self.next = usize::MAX;
//                     return None;
//                 }

//                 continue;
//             }

//             while (v & mask) != 0 {
//                 mask <<= 1;
//                 self.next += 1;
//                 if self.next >= self.bm.size {
//                     return None;
//                 }
//             }

//             break;
//         }

//         let resp = self.next;
//         self.next += 1;

//         Some(resp)
//     }
// }

#[derive(Hash, Eq, PartialEq, Clone)]
struct Reindeer {
    idx: i32,
    dir: Dir,
}

impl Reindeer {
    fn new(idx: i32, d: Dir) -> Reindeer {
        Reindeer { idx: idx, dir: d }
    }
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn all() -> &'static [Dir; 4] {
        &[Dir::Left, Dir::Right, Dir::Up, Dir::Down]
    }

    fn translations() -> &'static [(i32, i32); 4] {
        &[(-1, 0), (1, 0), (0, -1), (0, 1)]
    }

    fn as_translation(self) -> (i32, i32) {
        Dir::translations()[self as usize]
    }

    fn rotate_180_degrees(self) -> Dir {
        [Dir::Right, Dir::Left, Dir::Down, Dir::Up][self as usize]
    }
}

impl From<u8> for Dir {
    fn from(v: u8) -> Self {
        Dir::all()[v as usize]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let w = INPUT.find("\n").expect("no newlines") - 2;
    let h = INPUT.trim_end().chars().filter(|c| *c == '\n').count() - 1;
    let idx_upper_bound = w * h;

    let idx_to_point = |i: usize| -> (i32, i32) { ((i % w) as i32, (i / w) as i32) };
    let point_to_idx = |p: (i32, i32)| -> usize { p.1 as usize * w + p.0 as usize };

    let (num_verts, edges, start_idx, end_idx) = {
        let mut edges = HashMap::<Reindeer, HashSet<(i32, Reindeer)>>::new();
        let mut start_idx: i32 = -1;
        let mut end_idx: i32 = -1;
        let mut vertex_bm = Bitmap::new(idx_upper_bound);
        let mut walls = Bitmap::new(idx_upper_bound);
        let mut visited: Bitmap = Bitmap::new(idx_upper_bound);
        let mut idx = 0;
        for mut v in (&INPUT[w + 3..(INPUT.len() - w - 3)]).lines() {
            v = &v[1..w + 1];
            for v in v.as_bytes().iter().cloned() {
                match v {
                    b'S' => start_idx = idx,
                    b'E' => end_idx = idx,
                    b'#' => {
                        walls.set(idx as usize);
                    }
                    _ => {}
                }
                idx += 1;
            }
        }

        vertex_bm.set(start_idx as usize);
        vertex_bm.set(end_idx as usize);

        for idx in 0..idx_upper_bound {
            if walls.is_set(idx) {
                continue;
            }

            let mut num_options = 0;
            for d in Dir::translations() {
                let np = idx_to_point(idx);
                let np = (np.0 + d.0, np.1 + d.1);
                if np.0 < 0 || np.1 < 0 || np.0 >= w as i32 || np.1 >= h as i32 {
                    continue;
                }

                if walls.is_set(point_to_idx(np)) {
                    continue;
                }

                num_options += 1;
            }

            // anything less than 2 is a dead end and can be ignored

            if num_options > 2 {
                vertex_bm.set(idx);
            }
        }

        // construct path lengths
        let mut dbuf = Vec::<Dir>::with_capacity(4);
        for idx in vertex_bm.iter_set() {
            let p = idx_to_point(idx);
            dbuf.clear();

            for (di, d) in Dir::translations().iter().cloned().enumerate() {
                let np = (p.0 + d.0, p.1 + d.1);
                if np.0 < 0
                    || np.1 < 0
                    || np.0 as usize >= w
                    || np.1 as usize >= h
                    || walls.is_set(point_to_idx(np))
                {
                    continue;
                }
                dbuf.push(Dir::from(di as u8));
            }

            for enter_dir in dbuf.iter().cloned() {
                // calculate the score of this segment without an initial starting direction
                // and find the end of it
                //
                // note that there can be dead ends - ignore them
                let mut nd = enter_dir;
                let mut nidx = {
                    let d = enter_dir.as_translation();
                    point_to_idx((p.0 + d.0, p.1 + d.1)) as i32
                };
                visited.clear();
                visited.set(idx);
                visited.set(nidx as usize);
                let mut score = 1;
                let mut dead_end = false;
                while !dead_end && !vertex_bm.is_set(nidx as usize) {
                    score += 1;
                    dead_end = true;
                    for (i, d) in Dir::translations().iter().cloned().enumerate() {
                        let p = {
                            let p = idx_to_point(nidx as usize);
                            (p.0 + d.0, p.1 + d.1)
                        };

                        if p.0 < 0 || p.1 < 0 || p.0 as usize >= w || p.1 as usize >= h {
                            continue;
                        }

                        let idx = point_to_idx(p);
                        if walls.is_set(idx) || !visited.set(idx) {
                            continue;
                        }

                        dead_end = false;
                        let d = Dir::from(i as u8);
                        if d != nd {
                            score += if d.rotate_180_degrees() == nd {
                                2000
                            } else {
                                1000
                            };
                            nd = d;
                        }
                        nidx = idx as i32;

                        break;
                    }
                }
                if dead_end {
                    continue;
                }

                // we've arrived at a vertex
                for v in dbuf.iter().cloned() {
                    let mut score = score;
                    let prev_dir = if (idx as i32) == start_idx {
                        if enter_dir == Dir::Left {
                            // note this block is not needed if start is always at the bottom left
                            score += 1000;
                        }
                        Dir::Right
                    } else {
                        v.rotate_180_degrees()
                    };
                    if prev_dir != enter_dir {
                        score += 1000
                    }
                    let src = Reindeer::new(idx as i32, prev_dir);
                    let scored_dest = (score, Reindeer::new(nidx, nd));
                    if let Some(v) = edges.get_mut(&src) {
                        v.insert(scored_dest);
                        continue;
                    }

                    let mut hs = HashSet::<(i32, Reindeer)>::new();
                    hs.insert(scored_dest);
                    edges.insert(src, hs);
                }
            }
        }

        (vertex_bm.count_ones(), edges, start_idx, end_idx)
    };

    let sum = {
        // visited maps the end state index to the score and dir
        let mut visited = HashMap::<i32, (i32, Dir)>::with_capacity(num_verts);
        let mut origins = HashMap::<i32, (i32, Dir)>::with_capacity(num_verts);
        let mut exhausted_origins = Vec::<i32>::with_capacity(num_verts);
        {
            let origin = (0, Dir::Right);
            visited.insert(start_idx, origin);
            origins.insert(start_idx, origin);
        }
        while !visited.contains_key(&end_idx) {
            // (new total score, new end state)
            let mut next: Option<(i32, Reindeer)> = None;

            // for each origin
            // find a new minimal path change destination that adds a new node
            // if nothing could be found then remove that origin from the processing set
            for (k, v) in origins.iter() {
                let (origin, (prev_score, prev_dir)) = (*k, *v);

                let prev_state = Reindeer::new(origin, prev_dir);

                let edges = if let Some(v) = edges.get(&prev_state) {
                    v
                } else {
                    exhausted_origins.push(origin);
                    continue;
                };

                let mut it = edges.iter().cloned();
                let mut found_edge = false;
                loop {
                    let (score_inc, end_state) = if let Some(v) = it.next() {
                        v
                    } else {
                        break;
                    };

                    if visited.contains_key(&end_state.idx) {
                        continue;
                    }

                    let score = prev_score + score_inc;
                    let old_next = if let Some(v) = &next {
                        v
                    } else {
                        next = Some((score, end_state));

                        found_edge = true;
                        break;
                    };

                    if score < old_next.0 {
                        next = Some((score, end_state));
                    }

                    found_edge = true;
                    break;
                }
                if !found_edge {
                    exhausted_origins.push(origin);
                    continue;
                }

                for (score_inc, end_state) in it {
                    if visited.contains_key(&end_state.idx) {
                        continue;
                    }

                    let old_next = if let Some(v) = &next {
                        v
                    } else {
                        next = Some((prev_score + score_inc, end_state));
                        continue;
                    };

                    let score = prev_score + score_inc;
                    if score < old_next.0 {
                        next = Some((score, end_state));
                    }
                }
            }

            let next = if let Some(v) = next {
                v
            } else {
                // failed to find a next node to add to the visited set
                //
                // stop processing
                break;
            };

            for v in exhausted_origins.iter().cloned() {
                origins.remove(&v);
            }
            exhausted_origins.clear();

            let (end_score, end_state) = next;

            let v = (end_score, end_state.dir);
            origins.insert(end_state.idx, v);
            visited.insert(end_state.idx, v);
        }

        visited.get(&end_idx).cloned().expect("no solution found").0
    };

    println!("{}", sum);
    Ok(())
}
