use std::{cell::RefCell, collections::HashMap, error::Error};

const INPUT: &str = include_str!("../../../input.txt");

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

    fn set<T: Into<usize>>(&mut self, idx: T) {
        let idx = idx.into();

        self.0[idx / 8] |= 1 << (idx % 8);
    }

    fn count_ones(&self) -> usize {
        let mut sum = 0;
        for v in &self.0 {
            sum += v.count_ones() as usize;
        }
        sum
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let w = INPUT.find("\n").expect("no newlines");
    let h = INPUT.trim_end().chars().filter(|c| *c == '\n').count() + 1;

    let mut bm = Bitmap::new(w * h);
    let mut m = HashMap::<u8, RefCell<Vec<(i32, i32)>>>::new();

    let mut y = -1;
    for v in INPUT.lines() {
        y += 1;
        let mut x = -1;
        for v in v.as_bytes() {
            x += 1;
            if *v == b'.' {
                continue;
            }
            match m.get(v) {
                Some(v) => {
                    v.borrow_mut().push((x, y));
                }
                None => {
                    let mut list = Vec::<(i32, i32)>::with_capacity(2);
                    list.push((x, y));
                    m.insert(*v, RefCell::new(list));
                }
            }
        }
    }

    for v in m.values() {
        let mut v = v.borrow_mut();
        while v.len() > 1 {
            let p1 = v.pop().expect("unreachable");
            for p2 in v.iter() {
                let d = (p2.0 - p1.0, p2.1 - p1.1);
                let anti = (p1.0 - d.0, p1.1 - d.1);
                if anti.0 >= 0 && anti.0 < w as i32 && anti.1 >= 0 && anti.1 < h as i32 {
                    bm.set(anti.1 as usize * w + anti.0 as usize);
                }
                let anti = (p2.0 + d.0, p2.1 + d.1);
                if anti.0 >= 0 && anti.0 < w as i32 && anti.1 >= 0 && anti.1 < h as i32 {
                    bm.set(anti.1 as usize * w + anti.0 as usize);
                }
            }
        }
    }

    println!("{}", bm.count_ones());
    Ok(())
}
